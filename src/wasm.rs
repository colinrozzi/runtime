use thiserror::Error;
use wasmtime::component::ComponentExportIndex;
use wasmtime::component::{Component, Instance, Linker};
use wasmtime::{Engine, Store, StoreContextMut};

#[derive(Error, Debug)]
pub enum WasmComponentError {
    #[error("WASM initialization failed: {0}")]
    InitError(String),
    #[error("WASM message handling failed: {0}")]
    HandleError(String),
}

pub struct WasmComponent {
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

    pub async fn init(&mut self) -> Result<(), WasmComponentError> {
        let init_func = self
            .instance
            .get_func(&mut self.store, self.init_index)
            .expect("Failed to get init function");

        let typed = init_func
            .typed::<(), ()>(&self.store)
            .map_err(|e| WasmComponentError::InitError(e.to_string()))?;
        typed
            .call(&mut self.store, ())
            .map_err(|e| WasmComponentError::InitError(e.to_string()))?;
        Ok(())
    }

    pub async fn handle(&mut self, msg: &str, state: &str) -> Result<(), WasmComponentError> {
        let handle_func = self
            .instance
            .get_func(&mut self.store, self.handle_index)
            .expect("Failed to get handle function");

        let typed = handle_func
            .typed::<(&str, &str), ()>(&self.store)
            .map_err(|e| WasmComponentError::HandleError(e.to_string()))?;
        typed
            .call(&mut self.store, (msg, state))
            .map_err(|e| WasmComponentError::HandleError(e.to_string()))?;
        Ok(())
    }
}
