mod asn_base_engine_impl;
mod asn_event_emitter;

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

impl Engine {
    pub fn new() -> Self {
        trace!("Engine:new");
        Engine::default()
    }
    pub fn init(&mut self) {
        trace!("Engine:init")
    }
}
