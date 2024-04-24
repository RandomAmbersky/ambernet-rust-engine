mod engine;
mod engine_interface;

use crate::engine_interface::EngineInterface;
use asn_logger::{info, AsnLogLevel};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn init() -> EngineInterface {
    asn_logger::init_log(AsnLogLevel::Trace);
    info!("Hello world!");
    EngineInterface::default()
}
