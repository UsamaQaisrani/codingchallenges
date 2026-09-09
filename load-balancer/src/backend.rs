use std::sync::Arc;

use axum::{Router, extract::State, response::IntoResponse, routing::get};

struct AppState {
    id: i32,
}

pub async fn start(id: i32, port: u32) -> Result<(), anyhow::Error> {
    let shared_state = Arc::new(AppState { id });
    let app = Router::new()
        .route("/", get(hello))
        .with_state(shared_state);
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    println!("Backend Server #{} Listening on {}", id, port);
    axum::serve(listener, app).await?;
    Ok(())
}

async fn hello(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    format!("Hello from Backend Server # {}\n", state.id)
}
