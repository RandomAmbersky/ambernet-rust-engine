extern crate asn_core;

use crate::t_asn_bus::AsnReceiver;
use crate::t_asn_bus::AsnTransmitter;
use asn_core::asn_event::AsnEvent;

pub trait AsnModule {
    fn init(
        &self,
        t: impl AsnTransmitter<AsnEvent>,
        r: impl AsnReceiver<AsnEvent>,
    ) -> Result<(), String>;
    fn run(&self) -> Result<(), String>;
}
