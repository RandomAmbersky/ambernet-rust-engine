use asn_core::events::{AsnEvent, AsnEventEmitter};
use asn_logger::trace;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

use asn_core::traits::TAsnBaseEngine;

#[derive(Default, Debug)]
struct EngineState {
    is_need_exit: bool,
    events: VecDeque<AsnEvent>,
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

impl AsnEventEmitter for Engine {
    fn emit(&mut self, e: AsnEvent) -> Result<(), String> {
        self.state.lock().unwrap().events.push_back(e);
        Ok(())
    }
    fn pull(&mut self) -> Option<AsnEvent> {
        self.state.lock().unwrap().events.pop_front()
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
