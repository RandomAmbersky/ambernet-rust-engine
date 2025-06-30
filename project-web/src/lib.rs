extern crate asn_logger;

mod asn_web_pool;

use asn_logger::{trace, AsnLogConfig, AsnLogLevel};
use asn_web_pool::{new_asn_web_pool, AsnWebPool};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn console_log(l: String, m: String) {
    trace(l.as_str(), m.as_str());
}

#[wasm_bindgen]
pub fn get_engine() -> AsnWebPool {
    let c = AsnLogConfig {
        global_level: AsnLogLevel::Trace,
        module_levels: Default::default(),
    };
    asn_logger::init_log(&c);
    new_asn_web_pool()
}

// #[wasm_bindgen]
// pub fn greet() {
//     alert("Hello, {{project-name}}!");
// }
