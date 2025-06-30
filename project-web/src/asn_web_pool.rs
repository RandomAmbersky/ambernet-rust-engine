use asn_logger::{debug, error, info, trace, warn, AsnLogLevel};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct AsnWebPool {}

#[wasm_bindgen]
impl AsnWebPool {
    pub fn run(&mut self) {
        trace("AsnWebPool", " AsnWebPool run...");
    }
    pub fn log(&self, l: String, t: String, m: String) {
        let lv = AsnLogLevel::from_string(l.clone());

        if lv.is_err() {
            warn(
                "AsnWebPool",
                &format!("AsnLogLevel::from_string cant convert from '{l}'"),
            );
        }

        match lv.unwrap_or(AsnLogLevel::Off) {
            AsnLogLevel::Off => {}
            AsnLogLevel::Error => error(&t, &m),
            AsnLogLevel::Warn => warn(&t, &m),
            AsnLogLevel::Info => info(&t, &m),
            AsnLogLevel::Debug => debug(&t, &m),
            AsnLogLevel::Trace => trace(&t, &m),
        }
    }
}

#[wasm_bindgen]
pub fn new_asn_web_pool() -> AsnWebPool {
    AsnWebPool {}
}
