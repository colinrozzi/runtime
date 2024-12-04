use axum::{extract::State, http::StatusCode, routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::Runtime;

#[derive(Clone)]
pub struct SharedRuntime(Arc<Mutex<Runtime>>);

#[derive(Debug, Deserialize)]
pub struct Message {
    data: Value,
}

#[derive(Debug, Serialize)]
pub struct Response {
    hash: String,
    state: Value,
}

pub async fn serve(runtime: Runtime, addr: SocketAddr) -> anyhow::Result<()> {
    let shared = SharedRuntime(Arc::new(Mutex::new(runtime)));

    let app = Router::new()
        .route("/", post(handle_message))
        .with_state(shared);

    println!("Starting server on {}", addr);
    axum::serve(tokio::net::TcpListener::bind(addr).await?, app).await?;

    Ok(())
}

async fn handle_message(
    State(shared): State<SharedRuntime>,
    Json(message): Json<Message>,
) -> Result<Json<Response>, (StatusCode, String)> {
    let mut runtime = shared.0.lock().await;

    match runtime.handle_message(message.data).await {
        Ok((hash, state)) => Ok(Json(Response { hash, state })),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
    }
}
