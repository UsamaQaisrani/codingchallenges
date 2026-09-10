use crate::error::AppError;
use axum::{
    Router,
    extract::{Request, State},
    http::HeaderMap,
    response::IntoResponse,
    routing::any,
};
use reqwest::{Body, Method, Url};
use std::{
    sync::{Arc, Mutex},
    usize::MAX,
};

struct AppState {
    backends: Vec<String>,
    curr_idx: Mutex<usize>,
}

pub async fn start(backends: Vec<String>, port: u32) -> Result<(), anyhow::Error> {
    let curr_idx = Mutex::new(0);
    let shared_state = Arc::new(AppState { backends, curr_idx });
    let app = Router::new()
        .route("/{*path}", any(forward))
        .with_state(shared_state);
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

    let idx = *state.curr_idx.lock().unwrap();
    let backend_addr = &state.backends[idx];
    let mut url_string = format!("http://{}{}", backend_addr, path);

    if let Some(query) = req.uri().query() {
        url_string.push('?');
        url_string.push_str(query);
    }

    let body = req.into_body();

    let url: Url = url_string.parse()?;

    let body_bytes = axum::body::to_bytes(body, MAX).await?;
    let req_response = send_request(method.clone(), url, headers.clone(), body_bytes).await?;

    {
        let mut idx = state.curr_idx.lock().unwrap();
        *idx += 1;
        if *idx >= state.backends.len() {
            *idx = 0;
        }
    }

    let status = req_response.status();
    let headers = req_response.headers().clone();
    let body = req_response.bytes().await?;

    Ok((status, headers, body))
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
