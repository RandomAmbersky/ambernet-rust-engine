use asn_logger::AsnLogLevel;

pub struct Engine {}

impl Engine {
    pub fn run(&mut self) {}
}

pub fn init() -> Engine {
    let l: AsnLogLevel = AsnLogLevel::Trace;
    asn_logger::init_log(l);
    Engine {}
}
