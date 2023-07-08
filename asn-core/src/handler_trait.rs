use crate::{AsnContextTrait, AsnError, AsnEvent};

pub trait AsnHandlerTrait<A>
where
    A: AsnContextTrait,
{
    fn proceed(&mut self, ctx: &mut A, evt: &AsnEvent) -> Option<AsnError>;
}
