use crate::handler::Handler;

use asn_logger::trace;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub struct EngineInterface {
    h: Handler,
}

impl Default for EngineInterface {
    fn default() -> EngineInterface {
        EngineInterface { h: Handler::new() }
    }
}

// интерфейс для вызова и обработки событий с веб-страницы
// on-resize например
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl EngineInterface {
    pub fn run(&mut self) {
        let mut e = asn_engine_released::get_engine();
        let mut r = asn_winit_released::new_runner_dataset(&mut e, &mut self.h);
        asn_winit_released::run_loop(&mut r);
    }
    fn resize(&mut self) {}
}
