use asn_logger::AsnLogLevel;

pub fn init() {
    let l: AsnLogLevel = AsnLogLevel::Trace;
    asn_logger::init_log(l);
}
