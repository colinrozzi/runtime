use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use serde_json::json;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{Actor, ActorError, ActorMessage};

#[derive(Clone)]
pub struct ActorState {
    inner: Arc<Mutex<Actor>>,
}

pub async fn start_server(actor: Actor) -> Result<(), ActorError> {
    let shared_state = ActorState {
        inner: Arc::new(Mutex::new(actor)),
    };

    let app = Router::new()
        .route("/message", post(handle_message))
        .route("/status", get(get_status))
        .route("/state", get(get_state))
        .route("/hashchain", get(get_hashchain))
        .with_state(shared_state.clone());

    let address = shared_state.inner.lock().await.address;
    println!("Actor server starting on {}", address);

    axum_server::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .map_err(|e| ActorError::ServerError(e.to_string()))?;

    Ok(())
}

async fn handle_message(
    State(state): State<ActorState>,
    Json(message): Json<ActorMessage>,
) -> Json<serde_json::Value> {
    let mut actor = state.inner.lock().await;
    match actor.handle(&message.content, &message.state).await {
        Ok(()) => Json(json!({
            "status": "success",
            "from": message.from,
            "last_hash": actor.get_last_hash()
        })),
        Err(e) => Json(json!({
            "status": "error",
            "error": e.to_string()
        })),
    }
}

async fn get_status(State(state): State<ActorState>) -> Json<serde_json::Value> {
    let actor = state.inner.lock().await;
    Json(json!({
        "status": "healthy",
        "name": actor.get_name(),
        "address": actor.address.to_string()
    }))
}

async fn get_state(State(state): State<ActorState>) -> Json<serde_json::Value> {
    let actor = state.inner.lock().await;
    Json(json!({
        "name": actor.get_name(),
        "pending_messages": actor.wasm_component.get_pending_messages()
    }))
}

async fn get_hashchain(State(state): State<ActorState>) -> Json<serde_json::Value> {
    let actor = state.inner.lock().await;
    Json(json!({
        "chain": actor.get_chain(),
        "latest_hash": actor.get_last_hash()
    }))
}

