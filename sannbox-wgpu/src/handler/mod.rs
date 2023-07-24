use crate::engine::core::events::AsnEvent;
use crate::engine::core::traits::{TAsnHandler, TAsnWinapi};
use crate::engine::{Engine, NodeQuad, TAsnEngine};

pub struct Handler {
    quad: NodeQuad,
}

impl Handler {
    pub fn new(e: &mut Engine) -> Self {
        let quad = e.get_winapi().new_quad();
        Handler { quad }
    }
}

impl TAsnHandler<Engine> for Handler {
    fn handle(&mut self, evt: &AsnEvent, e: &mut Engine) {
        println!("handle {:?} event", &evt);
    }
}
