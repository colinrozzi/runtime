use anyhow::{Context, Result};
use serde_json::Value;
use sha2::{Digest, Sha256};
use wasmtime::component::*;
use wasmtime::{Config, Engine, Store};

// Host implementation of runtime interface
wit_bindgen::generate!({
    world: "first-actor",
    path: "wit"
});

#[derive(Default)]
struct RuntimeImpl {
    messages: Vec<String>,
}

impl runtime::Host for RuntimeImpl {
    fn log(&mut self, msg: &str) {
        println!("[WASM] {}", msg);
    }

    fn send(&mut self, actor_id: &str, msg: &Value) {
        self.messages.push(format!("To {}: {:?}", actor_id, msg));
    }
}

pub struct WasmComponent {
    store: Store<RuntimeImpl>,
    instance: ComponentInstance,
}

impl WasmComponent {
    pub fn new(wasm_path: &str) -> Result<(Self, String)> {
        // Read and hash component
        let wasm_bytes = std::fs::read(wasm_path).context("Failed to read WASM file")?;
        let mut hasher = Sha256::new();
        hasher.update(&wasm_bytes);
        let component_hash = format!("{:x}", hasher.finalize());

        // Set up wasmtime
        let mut config = Config::new();
        config.wasm_component_model(true);
        let engine = Engine::new(&config)?;
        let component = Component::new(&engine, &wasm_bytes)?;
        let mut linker = Linker::new(&engine);
        runtime::add_to_linker(&mut linker, |state: &mut RuntimeImpl| state)?;

        // Create store and instance
        let mut store = Store::new(&engine, RuntimeImpl::default());
        let instance = linker.instantiate(&mut store, &component)?;

        Ok((Self { store, instance }, component_hash))
    }

    pub fn init(&mut self) -> Result<Value> {
        let actor = Actor::new(&mut self.store, &self.instance)?;
        actor
            .init(&mut self.store)
            .context("Failed to initialize actor")
    }

    pub fn handle(&mut self, msg: Value, state: Value) -> Result<Value> {
        let actor = Actor::new(&mut self.store, &self.instance)?;

        // Verify message and state against contracts
        if !actor.message_contract(&mut self.store, &msg, &state)? {
            anyhow::bail!("Message failed contract verification");
        }
        if !actor.state_contract(&mut self.store, &state)? {
            anyhow::bail!("State failed contract verification");
        }

        // Handle message
        actor
            .handle(&mut self.store, msg, state)
            .context("Failed to handle message")
    }
}

