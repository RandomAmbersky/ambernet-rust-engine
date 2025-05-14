use asn_core_bus::{AsnBus, AsnModule, AsnModulePool};

pub struct Pool<B, M>
where
    B: AsnBus<M>,
{
    bus: B,
    _phantom: std::marker::PhantomData<M>,
}

impl<B, M> Pool<B, M>
where
    B: AsnBus<M>,
{
    fn new(bus: B) -> Pool<B, M> {
        Pool {
            bus,
            _phantom: Default::default(),
        }
    }
}

impl<B, M> AsnModulePool<B, M> for Pool<B, M>
where
    B: AsnBus<M>,
{
    fn add_module(&self, m: impl AsnModule<M>) -> Result<(), String> {
        let t = self.bus.get_sender();
        let r = self.bus.get_receiver();
        m.init(t, r).unwrap();
        Ok(())
    }
}

pub fn new_module_pool<B, M>(bus: B) -> impl AsnModulePool<B, M>
where
    B: AsnBus<M>,
{
    Pool::new(bus)
}
