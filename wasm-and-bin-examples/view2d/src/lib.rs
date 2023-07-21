mod library;
mod wasm_utils;

use asn_core::AsnEngineTrait;
pub use wasm_utils::greet;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use asn_logger::{info, AsnLogLevel};
use library::Engine;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn start() {
    asn_logger::init_log(AsnLogLevel::Debug);
    let mut e = Engine::new();
    info!("It worked :)");
    e.run();
}
