use crate::engine::core::events::AsnEvent;
use crate::engine::core::traits::TAsnHandler;
use crate::engine::Engine;

pub struct Handler {}

impl Handler {
    pub fn new() -> Self {
        Handler {}
    }
}

impl TAsnHandler<Engine> for Handler {
    fn handle(&mut self, evt: &AsnEvent, e: &mut Engine) {
        println!("handle {:?} event", &evt);
    }
}
