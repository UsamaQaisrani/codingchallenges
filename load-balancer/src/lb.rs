use crate::error::AppError;
use axum::{
    Router,
    extract::{Request, State},
    http::HeaderMap,
    response::IntoResponse,
};
use reqwest::{Body, Method, Url};
use std::sync::{Arc, Mutex};
use tokio::time;

struct AppState {
    active_backends: Mutex<Vec<String>>,
    inactive_backends: Mutex<Vec<String>>,
    curr_idx: Mutex<usize>,
}

pub async fn start(active_backends: Vec<String>, port: u32) -> Result<(), anyhow::Error> {
    let curr_idx = Mutex::new(0);
    let active_backends = Mutex::new(active_backends);
    let inactive_backends = Mutex::new(Vec::new());
    let shared_state = Arc::new(AppState {
        active_backends,
        inactive_backends,
        curr_idx,
    });

    tokio::spawn(health_check(Arc::clone(&shared_state)));

    let app = Router::new().fallback(forward).with_state(shared_state);
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    println!("Load Balancer Listening on {}", port);
    axum::serve(listener, app).await?;
    Ok(())
}

async fn forward(
    State(state): State<Arc<AppState>>,
    req: Request,
) -> Result<impl IntoResponse, AppError> {
    let method = req.method().clone();
    let path = req.uri().path().to_string();
    let headers = req.headers().clone();

    let (backend_addr, be_len) = {
        let active_bes = state.active_backends.lock().unwrap();

        if active_bes.is_empty() {
            return Err(anyhow::anyhow!("No backends available").into());
        }

        let mut idx = state.curr_idx.lock().unwrap();

        if *idx >= active_bes.len() {
            *idx = 0;
        }

        let backend_addr = active_bes[*idx].clone();

        (backend_addr, active_bes.len())
    };
    let mut url_string = format!("http://{}{}", backend_addr, path);

    if let Some(query) = req.uri().query() {
        url_string.push('?');
        url_string.push_str(query);
    }

    let body = req.into_body();

    let url: Url = url_string.parse()?;

    let body_bytes = axum::body::to_bytes(body, usize::MAX).await?;
    let req_response = send_request(method.clone(), url, headers.clone(), body_bytes).await?;

    {
        let mut idx = state.curr_idx.lock().unwrap();
        *idx += 1;
        if *idx >= be_len {
            *idx = 0;
        }
    }

    let status = req_response.status();
    let headers = req_response.headers().clone();
    let body = req_response.bytes().await?;

    Ok((status, headers, body))
}

async fn health_check(state: Arc<AppState>) {
    let mut interval = time::interval(time::Duration::from_secs(60));
    let client = reqwest::Client::new();

    loop {
        // Find inactive servers in active servers
        interval.tick().await;
        let mut unhealthy = Vec::new();

        let backends = {
            let active_bes = state.active_backends.lock().unwrap();
            active_bes.clone()
        };

        for be in &backends {
            let healthy = match client
                .request(Method::GET, format!("http://{}/health", *be))
                .send()
                .await
            {
                Ok(response) => response.status().is_success(),
                Err(_) => false,
            };

            if !healthy {
                unhealthy.push(be.clone());
            }
        }

        {
            let mut active_backends = state.active_backends.lock().unwrap();
            active_backends.retain(|be| !unhealthy.contains(be));
        }

        {
            let mut inactive_bes = state.inactive_backends.lock().unwrap();
            for be in unhealthy {
                inactive_bes.push(be);
            }
        }

        let inactive_bes = {
            let inactive_bes = state.inactive_backends.lock().unwrap();
            inactive_bes.clone()
        };

        let mut healthy = Vec::new();

        for be in &inactive_bes {
            let is_healthy = match client
                .request(Method::GET, format!("http://{}/health", *be))
                .send()
                .await
            {
                Ok(response) => response.status().is_success(),
                Err(_) => false,
            };

            if is_healthy {
                healthy.push(be.clone());
            }
        }

        {
            let mut inactive_bes = state.inactive_backends.lock().unwrap();
            inactive_bes.retain(|be| !healthy.contains(be));
        }

        {
            let mut active_bes = state.active_backends.lock().unwrap();
            for be in healthy {
                active_bes.push(be);
            }
        }
    }
}

async fn send_request(
    method: Method,
    path: Url,
    headers: HeaderMap,
    body: impl Into<Body>,
) -> Result<reqwest::Response, AppError> {
    let res_body = reqwest::Client::new()
        .request(method.clone(), path)
        .headers(headers.clone())
        .body(body)
        .send()
        .await?;

    Ok(res_body)
}
