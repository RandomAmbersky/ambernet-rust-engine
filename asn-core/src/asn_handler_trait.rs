use crate::{AsnError, AsnEvent, AsnWinapiTrait};

pub trait AsnHandlerTrait<'a, W, A>
where
    W: AsnWinapiTrait,
{
    fn proceed(&mut self, ctx: &mut A, evt: &AsnEvent) -> Option<AsnError>;
}
