use asn_core::events::AsnEvent;
use asn_logger::trace;
use std::sync::{Arc, Mutex};

use asn_core::traits::{TAsnBaseEngine, TAsnEventEngine};

#[derive(Default, Debug)]
struct EngineState {
    is_need_exit: bool,
}

#[derive(Default, Debug)]
pub struct Engine {
    state: Arc<Mutex<EngineState>>,
}

impl TAsnBaseEngine for Engine {
    fn is_need_exit(&self) -> bool {
        let s = self.state.lock().unwrap();
        s.is_need_exit
    }
    fn set_need_exit(&mut self) {
        let mut s = self.state.lock().unwrap();
        s.is_need_exit = true
    }
}

impl TAsnEventEngine<AsnEvent> for Engine {
    fn emit(&mut self, e: &AsnEvent) -> Result<(), String> {
        todo!()
    }
}

impl Engine {
    pub fn new() -> Self {
        trace!("Engine:new");
        Engine::default()
    }
    pub fn init(&mut self) {
        trace!("Engine:init")
    }
}
