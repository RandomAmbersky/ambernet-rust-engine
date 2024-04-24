use crate::engine::TAsnEngineHandler;
use asn_core::events::AsnEvent;
use asn_core::traits::TAsnBaseEngine;
use asn_logger::trace;

pub struct Handler {}

impl Handler {
    pub fn new() -> Self {
        Handler {}
    }
}

impl<E> TAsnEngineHandler<E> for Handler
where
    E: TAsnBaseEngine,
{
    fn handle(&mut self, evt: &AsnEvent, engine: &mut E) {
        trace!("Handler handle: {:?} {:?}", evt, engine.is_need_exit())
    }
}
