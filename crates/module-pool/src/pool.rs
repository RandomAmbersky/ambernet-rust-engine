use asn_core::asn_event::AsnEvent;
use asn_core_bus::{AsnBus, AsnModule, AsnModulePool};

pub struct Pool<B>
where
    B: AsnBus<AsnEvent>,
{
    bus: B,
}

impl<B> Pool<B>
where
    B: AsnBus<AsnEvent>,
{
    fn new(bus: B) -> Pool<B> {
        Pool { bus }
    }
}

impl<B> AsnModulePool<B> for Pool<B>
where
    B: AsnBus<AsnEvent>,
{
    fn add_module(&self, m: impl AsnModule) -> Result<(), String> {
        let t = self.bus.get_sender();
        let r = self.bus.get_receiver();
        m.init(t, r).unwrap();
        Ok(())
    }
}

pub fn new_asn_module_pool<B>(bus: B) -> impl AsnModulePool<B>
where
    B: AsnBus<AsnEvent>,
{
    Pool::new(bus)
}
