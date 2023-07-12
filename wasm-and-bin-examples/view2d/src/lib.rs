mod library;
mod wasm_utils;

pub use wasm_utils::greet;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use asn_logger::{info, AsnLogLevel};

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn start() {
    asn_logger::init_log(AsnLogLevel::Debug);
    let mut e = library::Engine::new();
    info!("It worked :)");
    e.do_infinite_loop()
}
