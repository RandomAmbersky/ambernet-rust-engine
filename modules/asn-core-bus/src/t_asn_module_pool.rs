use crate::AsnModule;

pub trait AsnModulePool<M, Uid> {
    fn add_module(m: impl AsnModule<M>) -> Result<Uid, String>;
}
