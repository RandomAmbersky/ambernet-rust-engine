use crate::engine::Engine;
use asn_core::events::{AsnEvent, AsnEventEmitter};

impl AsnEventEmitter for Engine {
    fn emit(&mut self, e: AsnEvent) -> Result<(), String> {
        // trace!("emit -> {:?}", e);
        self.state.lock().unwrap().events.push_back(e);
        Ok(())
    }
    fn pull(&mut self) -> Option<AsnEvent> {
        self.state.lock().unwrap().events.pop_front()
    }
}
