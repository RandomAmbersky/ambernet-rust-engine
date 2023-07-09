use crate::{AsnContextTrait, AsnError, AsnEvent, AsnWinapiTrait};

pub trait AsnHandlerTrait<'a, W, A>
where
    W: AsnWinapiTrait,
    A: AsnContextTrait<'a, W>,
{
    fn proceed(&mut self, ctx: &mut A, evt: &AsnEvent) -> Option<AsnError>;
}
