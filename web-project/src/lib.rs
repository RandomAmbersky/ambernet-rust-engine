extern crate asn_logger;

mod asn_engine_errors;
mod asn_web_pool;
mod dto_web_messages;

use asn_logger::AsnLogLevel;
use asn_web_pool::{new_asn_web_pool, AsnWebPool};
use wasm_bindgen::prelude::*;

// #[wasm_bindgen]
// extern "C" {
//     fn alert(s: &str);
// }

#[wasm_bindgen]
pub fn get_engine() -> AsnWebPool {
    console_error_panic_hook::set_once();
    asn_logger::init_log(AsnLogLevel::Trace);
    new_asn_web_pool()
}

// #[wasm_bindgen]
// pub fn greet() {
//     alert("Hello, {{project-name}}!");
// }
