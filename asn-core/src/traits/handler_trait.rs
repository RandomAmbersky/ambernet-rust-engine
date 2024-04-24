use crate::events::AsnEvent;
use crate::traits::TAsnBaseEngine;

pub trait TAsnHandler<E> {
    fn handle(&mut self, evt: &AsnEvent, engine: &mut E);
}

pub trait TAsnEngineHandler<E>
where
    E: TAsnBaseEngine,
{
    fn handle(&mut self, evt: &AsnEvent, engine: &mut E);
}
