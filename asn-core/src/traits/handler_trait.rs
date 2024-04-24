use crate::events::AsnEvent;
use crate::traits::TAsnBaseEngine;

pub trait TAsnHandler<E> {
    fn handle(&mut self, evt: &AsnEvent, engine: &mut E);
}
