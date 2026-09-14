use clap::Parser;
use std::sync::Arc;

use axum::{Router, extract::State, response::IntoResponse, routing::get};

struct AppState {
    id: i32,
}

#[derive(Parser)]
struct Args {
    #[arg(short = 'p')]
    id: i32,

    port: u32,
}

#[tokio::main]
pub async fn main() {
    let args = Args::parse();
    let be = start(args.id, args.port);
    let _be = tokio::join!(be);
    println!("Backend #{}: 0.0.0.0:{}", args.id, args.port)
}

async fn start(id: i32, port: u32) -> Result<(), anyhow::Error> {
    let shared_state = Arc::new(AppState { id });
    let app = Router::new()
        .route("/", get(hello))
        .route("/health", get(health_check))
        .with_state(shared_state);
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    println!("Backend Server #{} Listening on {}", id, port);
    axum::serve(listener, app).await?;
    Ok(())
}

async fn health_check(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    format!("Backend Server #{}: HEALTHY\n", state.id)
}

async fn hello(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    format!("Hello from Backend Server # {}\n", state.id)
}
