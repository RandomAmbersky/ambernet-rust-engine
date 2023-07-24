use crate::engine::core::events::AsnEvent;
use crate::engine::core::traits::TAsnHandler;
use crate::engine::{Engine, TAsnEngine};

pub struct Handler {}

impl Handler {
    pub fn new(e: &mut Engine) -> Self {
        e.get_winapi();
        Handler {}
    }
}

impl TAsnHandler<Engine> for Handler {
    fn handle(&mut self, evt: &AsnEvent, e: &mut Engine) {
        println!("handle {:?} event", &evt);
    }
}
