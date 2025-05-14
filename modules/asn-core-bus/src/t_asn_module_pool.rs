use crate::AsnBus;
use crate::AsnModule;

pub trait AsnModulePool<B, M>
where
    B: AsnBus<M>,
{
    fn add_module(&self, m: impl AsnModule<M>) -> Result<(), String>;
}
