use crate::engine::core::events::AsnEvent;
use crate::engine::TAsnEngine;

pub trait TAsnHandler<E>
where
    E: TAsnEngine,
{
    fn handle(&mut self, evt: &AsnEvent, engine: &mut E);
}
