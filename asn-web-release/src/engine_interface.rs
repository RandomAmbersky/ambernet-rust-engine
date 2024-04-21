#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub struct EngineInterface {}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl EngineInterface {}
