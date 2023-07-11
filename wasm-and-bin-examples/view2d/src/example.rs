use asn_logger::{info, AsnLogLevel};

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn start() {
    let l: AsnLogLevel = AsnLogLevel::Trace;
    asn_logger::init_log(l);
    info!("It worked :)");
}
