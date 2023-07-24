use crate::engine::core::events::AsnEvent;
use crate::engine::core::traits::{TAsnHandler, TAsnWinapi};
use crate::engine::{Engine, TAsnEngine};

pub struct Handler {}

impl Handler {
    pub fn new() -> Self {
        Handler {}
    }
}

impl TAsnHandler<Engine> for Handler {
    fn handle(&mut self, evt: &AsnEvent, engine: &mut Engine) {
        println!("handle {:?} event", &evt);
        engine.get_winapi().redraw();
    }
}
