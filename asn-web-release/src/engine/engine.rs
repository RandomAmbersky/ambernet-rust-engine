use asn_core::errors::AsnError;
use asn_core::events::AsnEvent;
use asn_logger::trace;
use std::sync::{Arc, Mutex};

use crate::engine::handler::handle;
use asn_core::traits::{TAsnBaseEngine, TAsnHandleEngine};

struct EngineState {
    is_need_exit: bool,
}

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

impl Engine {
    pub fn new() -> Self {
        trace!("Engine:new");
        let state = EngineState {
            is_need_exit: false,
        };
        Engine {
            state: Arc::new(Mutex::new(state)),
        }
    }
    pub fn init(&mut self) {
        trace!("Engine:init")
    }
    pub fn run(&mut self) {
        trace!("Engine:run");
        asn_winit_released::run_loop(self);
    }
}

impl TAsnHandleEngine for Engine {
    fn handle(&mut self, e: &AsnEvent) -> Result<(), AsnError> {
        trace!("Engine:handle: {:?}", e);
        handle(e, self);
        Ok(())
    }
}
