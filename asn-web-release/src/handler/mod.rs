mod handle_function;

use crate::handler::handle_function::handle;
use asn_core::events::AsnEvent;
use asn_core::traits::{TAsnBaseEngine, TAsnHandler};
use asn_logger::trace;

#[derive(Default, Debug)]
pub struct Handler {}

impl Handler {
    pub fn new() -> Self {
        trace!("Handler:new");
        Self::default()
    }
}

impl<E> TAsnHandler<E> for Handler
where
    E: TAsnBaseEngine,
{
    fn handle(&mut self, evt: &AsnEvent, engine: &mut E) {
        trace!("Handler:handle {:?}", evt);
        handle(evt, engine)
    }
}
