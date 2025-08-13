extern crate asn_winapi_traits;

use crate::engine::{init, Engine};
use asn_logger::AsnLogLevel;

mod engine;
mod handler;

fn main() {
    asn_logger::init_log(AsnLogLevel::Trace);
    let mut e = init();
    e.run()
}
