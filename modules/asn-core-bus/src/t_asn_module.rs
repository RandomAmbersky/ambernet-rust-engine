extern crate asn_core;

use crate::t_asn_bus::AsnReceiver;
use crate::t_asn_bus::AsnTransmitter;

pub trait AsnModule<M> {
    fn init(&self, t: impl AsnTransmitter<M>, r: impl AsnReceiver<M>) -> Result<(), String>;
    fn run(&self) -> Result<(), String>;
}
