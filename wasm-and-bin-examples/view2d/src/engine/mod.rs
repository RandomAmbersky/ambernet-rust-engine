mod engine;
mod handler;

use crate::engine::engine::Engine;
use asn_logger::AsnLogLevel;

pub fn init() -> Engine {
    let l: AsnLogLevel = AsnLogLevel::Trace;
    asn_logger::init_log(l);
    Engine::new()
}
