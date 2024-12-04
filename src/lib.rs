use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};
use std::net::SocketAddr;
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::Mutex;
use wasmtime::component::ComponentExportIndex;
use wasmtime::component::{Component, Instance, Linker};
use wasmtime::{Engine, Store, StoreContextMut};

#[derive(Error, Debug)]
pub enum ActorError {
    #[error("WASM initialization failed: {0}")]
    WasmInitError(String),
    #[error("WASM message handling failed: {0}")]
    WasmHandleError(String),
    #[error("Message handling failed: {0}")]
    HandleError(String),
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

struct WasmComponent {
    wasm_path: String,
    engine: Engine,
    component: Component,
    instance: Instance,
    store: Store<()>,
    linker: Linker<()>,
    init_index: ComponentExportIndex,
    handle_index: ComponentExportIndex,
}

impl WasmComponent {
    pub fn new(wasm_path: String, actor_name: String) -> Self {
        let engine = Engine::default();
        let bytes = std::fs::read(wasm_path.clone()).expect("Failed to read wasm file");
        let component = Component::new(&engine, &bytes).expect("Failed to create component");

        let mut store = Store::new(&engine, ());

        let mut linker = Linker::new(&engine);
        let mut runtime = linker
            .instance("ntwk:simple-actor/runtime")
            .expect("Failed to get runtime instance");

        let name_copy = actor_name.clone();
        runtime
            .func_wrap(
                "log",
                move |ctx: StoreContextMut<'_, ()>, (log,): (String,)| {
                    println!("{}: {}", name_copy, log);
                    Ok(())
                },
            )
            .expect("Failed to wrap log function");

        runtime.func_wrap(
            "send",
            move |ctx: StoreContextMut<'_, ()>, (actor_id, msg): (String, String)| {
                println!("send [actor-id : {}] [msg : {}]", actor_id, msg);

                Ok(())
            },
        );

        let instance = linker
            .instantiate(&mut store, &component)
            .expect("Failed to instantiate");

        let (_, instance_index) = component
            .export_index(None, "ntwk:simple-actor/actor")
            .expect("Failed to get export index");

        let (_, init_index) = component
            .export_index(Some(&instance_index), "init")
            .expect("Failed to get export index for init");

        let (_, handle_index) = component
            .export_index(Some(&instance_index), "handle")
            .expect("Failed to get export index for handle");

        WasmComponent {
            wasm_path,
            engine,
            component,
            instance,
            store,
            linker,
            init_index,
            handle_index,
        }
    }

    pub async fn init(&mut self) -> Result<(), ActorError> {
        let init_func = self
            .instance
            .get_func(&mut self.store, self.init_index)
            .expect("Failed to get init function");

        let typed = init_func
            .typed::<(), ()>(&self.store)
            .map_err(|e| ActorError::WasmInitError(e.to_string()))?;
        typed
            .call(&mut self.store, ())
            .map_err(|e| ActorError::WasmInitError(e.to_string()))?;
        Ok(())
    }

    pub async fn handle(&mut self, msg: &str, state: &str) -> Result<(), ActorError> {
        let handle_func = self
            .instance
            .get_func(&mut self.store, self.handle_index)
            .expect("Failed to get handle function");

        let typed = handle_func
            .typed::<(&str, &str), ()>(&self.store)
            .map_err(|e| ActorError::WasmHandleError(e.to_string()))?;
        typed
            .call(&mut self.store, (msg, state))
            .map_err(|e| ActorError::WasmHandleError(e.to_string()))?;
        Ok(())
    }
}

struct HashChain {
    chain: Vec<String>,
}

impl HashChain {
    pub fn new() -> Self {
        HashChain { chain: vec![] }
    }

    pub fn add(&mut self, data: String) {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        data.hash(&mut hasher);
        let hash = format!("{:x}", hasher.finish());
        let print_hash = hash.clone();
        self.chain.push(hash);
        println!("Added to chain: {}: {}", print_hash, data);
    }

    pub fn get(&self) -> Vec<String> {
        self.chain.clone()
    }

    pub fn get_last(&self) -> String {
        self.chain.last().unwrap().clone()
    }
}
