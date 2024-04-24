use asn_engine_released::EngineRealize;

use crate::handler::Handler;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub struct EngineInterface {
    e: EngineRealize,
    h: Handler,
}

impl Default for EngineInterface {
    fn default() -> Self {
        EngineInterface {
            e: EngineRealize::new(),
            h: Handler::new(),
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl EngineInterface {
    pub fn run(&mut self) {
        asn_winit_released::run_loop(&mut self.e, &mut self.h)
    }
}
