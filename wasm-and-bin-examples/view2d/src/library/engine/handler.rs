use asn_core::{AsnContext, AsnError, AsnEvent, AsnHandlerTrait};

pub struct Handler {}

impl Handler {
    pub fn new() -> Self {
        Self {}
    }
}

impl AsnHandlerTrait for Handler {
    fn proceed(&mut self, ctx: &mut AsnContext, evt: &AsnEvent) -> Option<AsnError> {
        None
    }
}
