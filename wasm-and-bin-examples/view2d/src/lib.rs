mod library;
mod wasm_utils;

pub use wasm_utils::greet;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use asn_logger::info;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn start() {
    let mut e = library::Engine::new();
    info!("It worked :)");
    e.run()
}
