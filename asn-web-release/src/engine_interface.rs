use crate::engine::Engine;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub struct EngineInterface {
    e: Engine,
}

impl Default for EngineInterface {
    fn default() -> Self {
        EngineInterface { e: Engine::new() }
    }
}
