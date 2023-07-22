use crate::{AsnContext, AsnError, AsnEvent};

pub trait AsnHandlerTrait {
    fn proceed(&mut self, ctx: &mut AsnContext, evt: &AsnEvent) -> Option<AsnError>;
}
