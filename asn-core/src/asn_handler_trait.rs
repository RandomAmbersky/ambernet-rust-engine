use crate::{AsnError, AsnEvent};

pub trait AsnHandlerTrait<C> {
    fn proceed(&mut self, ctx: &mut C, evt: &AsnEvent) -> Option<AsnError>;
}
