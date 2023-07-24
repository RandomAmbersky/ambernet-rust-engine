use crate::engine::core::events::AsnEvent;

pub trait TAsnHandler {
    fn handle(&mut self, evt: &AsnEvent);
}
