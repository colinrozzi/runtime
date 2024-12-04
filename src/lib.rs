use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::Mutex;

mod chain;
use chain::HashChain;

mod wasm;
use wasm::WasmComponent;

mod network;
use network::ActorState;

#[derive(Error, Debug)]
pub enum ActorError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Server error: {0}")]
    ServerError(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActorMessage {
    from: String,
    content: String,
    state: String,
}

#[derive(Clone)]
pub struct ActorState {
    inner: Arc<Mutex<Actor>>,
}

pub struct Actor {
    hash_chain: HashChain,
    actor_name: String,
    wasm_component: WasmComponent,
    address: SocketAddr,
}

impl Actor {
    pub fn new(
        wasm_path: String,
        actor_name: String,
        address: SocketAddr,
    ) -> Result<Self, ActorError> {
        Ok(Actor {
            hash_chain: HashChain::new(),
            actor_name: actor_name.clone(),
            wasm_component: WasmComponent::new(wasm_path, actor_name)?,
            address,
        })
    }

    pub async fn start_server(self) -> Result<(), ActorError> {
        let shared_state = ActorState {
            inner: Arc::new(Mutex::new(self)),
        };

        let app = Router::new()
            .route("/message", post(handle_message))
            .route("/status", get(get_status))
            .route("/state", get(get_state))
            .route("/hashchain", get(get_hashchain))
            .with_state(shared_state.clone());

        println!(
            "Actor server starting on {}",
            shared_state.inner.lock().await.address
        );

        axum_server::Server::bind(&shared_state.inner.lock().await.address)
            .serve(app.into_make_service())
            .await
            .map_err(|e| ActorError::ServerError(e.to_string()))?;

        Ok(())
    }

    pub async fn handle(&mut self, msg: &str, state: &str) -> Result<(), ActorError> {
        let record = format!("handle {} {}", msg, state);
        self.hash_chain.add(record);
        self.wasm_component
            .handle(msg, state)
            .await
            .map_err(|e| ActorError::HandleError(e.to_string()))?;
        Ok(())
    }

    pub fn get_chain(&self) -> &[String] {
        self.hash_chain.get()
    }

    pub fn get_last_hash(&self) -> Option<&String> {
        self.hash_chain.get_last()
    }

    pub fn get_name(&self) -> &str {
        &self.actor_name
    }
}

// HTTP endpoint handlers
async fn handle_message(
    State(state): State<ActorState>,
    Json(message): Json<ActorMessage>,
) -> Json<serde_json::Value> {
    let mut actor = state.inner.lock().await;
    match actor.handle(&message.content, &message.state).await {
        Ok(()) => Json(serde_json::json!({
            "status": "success",
            "from": message.from,
            "last_hash": actor.get_last_hash()
        })),
        Err(e) => Json(serde_json::json!({
            "status": "error",
            "error": e.to_string()
        })),
    }
}

async fn get_status(State(state): State<ActorState>) -> Json<serde_json::Value> {
    let actor = state.inner.lock().await;
    Json(serde_json::json!({
        "status": "healthy",
        "name": actor.get_name(),
        "address": actor.address.to_string()
    }))
}

async fn get_state(State(state): State<ActorState>) -> Json<serde_json::Value> {
    let actor = state.inner.lock().await;
    Json(serde_json::json!({
        "name": actor.get_name(),
        "pending_messages": actor.wasm_component.get_pending_messages()
    }))
}

async fn get_hashchain(State(state): State<ActorState>) -> Json<serde_json::Value> {
    let actor = state.inner.lock().await;
    Json(serde_json::json!({
        "chain": actor.get_chain(),
        "latest_hash": actor.get_last_hash()
    }))
}
