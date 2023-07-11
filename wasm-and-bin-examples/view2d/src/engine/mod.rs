mod context;

use crate::engine::context::Context;
use asn_logger::AsnLogLevel;

pub struct Engine {
    ctx: Context,
}

impl Engine {
    pub fn run(&mut self) {}
}

pub fn init() -> Engine {
    let l: AsnLogLevel = AsnLogLevel::Trace;
    asn_logger::init_log(l);
    let ctx = context::get_context();
    Engine { ctx }
}
