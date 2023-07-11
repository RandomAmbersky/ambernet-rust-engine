use asn_core::{AsnError, AsnEvent, AsnHandlerTrait};
use asn_logger::info;

pub struct Handler {}

impl Handler {
    pub fn new() -> Self {
        Handler {}
    }
}

pub fn get_handler() -> Handler {
    Handler::new()
}

impl AsnHandlerTrait for Handler {
    fn proceed(&mut self, evt: &AsnEvent) -> Option<AsnError> {
        info!("{:?}", evt);
        None
    }
}
