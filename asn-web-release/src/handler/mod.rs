mod handle_function;

use crate::handler::handle_function::handle;
use asn_core::events::{AsnEvent, AsnEventEmitter};
use asn_core::traits::{TAsnBaseEngine, TAsnHandler};
use asn_logger::trace;

#[derive(Default, Debug)]
pub struct Handler {}

impl<E> TAsnHandler<E> for Handler
where
    E: TAsnBaseEngine + AsnEventEmitter,
{
    fn handle(&mut self, evt: &AsnEvent, engine: &mut E) {
        // trace!("Handler:handle {:?}", evt);
        handle(evt, engine)
    }
}
