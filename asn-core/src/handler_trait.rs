use crate::{AsnContextTrait, AsnError, AsnEvent, AsnWinapiTrait};

pub trait AsnHandlerTrait<W, A>
where
    W: AsnWinapiTrait,
    A: AsnContextTrait<W>,
{
    fn proceed(&mut self, ctx: &mut A, evt: &AsnEvent) -> Option<AsnError>;
}
