use crate::engine::EngineRealize;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub struct EngineInterface {
    e: EngineRealize,
}

impl Default for EngineInterface {
    fn default() -> Self {
        EngineInterface {
            e: EngineRealize::new(),
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl EngineInterface {
    pub fn run(&mut self) {
        self.e.run();
    }
}
