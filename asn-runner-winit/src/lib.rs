mod winit_runner;

use crate::winit_runner::WinitRunner;
use asn_core::AsnHandlerTrait;

pub fn new() -> WinitRunner {
    WinitRunner::new()
}
