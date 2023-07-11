mod engine;
mod wasm_utils;

pub use wasm_utils::greet;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use asn_logger::info;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn start() {
    let e = engine::init();
    info!("It worked :)");
}
