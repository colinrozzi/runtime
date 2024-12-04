mod chain;
mod wasm;
mod network;

use anyhow::Result;
use serde_json::Value;
use std::net::SocketAddr;

use chain::HashChain;
use wasm::WasmComponent;

pub struct Runtime {
    chain: HashChain,
    wasm: WasmComponent,
    current_state: Option<Value>,
}

impl Runtime {
    pub fn new(wasm_path: &str) -> Result<Self> {
        let (wasm, component_hash) = WasmComponent::new(wasm_path)?;
        let mut chain = HashChain::new();
        chain.initialize(&component_hash);

        Ok(Self {
            chain,
            wasm,
            current_state: None,
        })
    }

    pub async fn start(self, addr: SocketAddr) -> Result<()> {
        // Initialize WASM component and store initial state
        let mut this = self;
        let initial_state = this.wasm.init()?;
        this.current_state = Some(initial_state.clone());
        this.chain.add(initial_state);

        // Start network server
        network::serve(this, addr).await
    }

    pub async fn handle_message(&mut self, message: Value) -> Result<(String, Value)> {
        let current_state = self.current_state.clone()
            .expect("State not initialized");

        // Handle message and get new state
        let new_state = self.wasm.handle(message.clone(), current_state)?;
        
        // Add to chain and update current state
        let hash = self.chain.add(new_state.clone());
        self.current_state = Some(new_state.clone());

        Ok((hash, new_state))
    }
}
