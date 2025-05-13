extern crate asn_core;

use crate::t_asn_bus::AsnBus;

pub trait AsnModule<B, E>
where
    B: AsnBus<E>,
{
    fn init(&self, bus: B) -> Result<(), String>;
    fn run_module(&self) -> Result<(), String>;
}
