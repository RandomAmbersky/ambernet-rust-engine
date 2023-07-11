use crate::{AsnError, AsnEvent};

pub trait AsnHandlerTrait {
    fn proceed(&mut self, evt: &AsnEvent) -> Option<AsnError>;
}
