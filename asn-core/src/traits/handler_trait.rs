use crate::events::AsnEvent;

pub trait TAsnHandler<C> {
    fn handle(&mut self, evt: &AsnEvent, context: &mut C);
}
