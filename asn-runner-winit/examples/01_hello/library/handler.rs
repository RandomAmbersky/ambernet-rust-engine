use asn_core::{AsnError, AsnEvent, AsnHandlerTrait};

pub struct Handler {}

impl Handler {
    pub fn new() -> Self {
        Handler {}
    }
}
impl AsnHandlerTrait for Handler {
    fn proceed(&mut self, evt: &AsnEvent) -> Option<AsnError> {
        None
    }
}

pub fn get_handler() -> Handler {
    Handler::new()
}
