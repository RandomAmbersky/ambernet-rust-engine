use crate::AsnBus;
use crate::AsnModule;
use asn_core::asn_event::AsnEvent;

pub trait AsnModulePool<B>
where
    B: AsnBus<AsnEvent>,
{
    fn add_module(&self, m: impl AsnModule) -> Result<(), String>;
}
