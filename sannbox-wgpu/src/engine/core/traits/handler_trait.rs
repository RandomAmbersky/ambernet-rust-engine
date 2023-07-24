use crate::engine::core::events::AsnEvent;

pub trait TAsnHandler<E> {
    fn handle(&mut self, evt: &AsnEvent, engine: &mut E);
}
