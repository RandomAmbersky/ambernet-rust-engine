use crate::asn_errors::AsnError;
use crate::AsnHandlerTrait;

pub trait AsnRunnerTrait<H>
where
    H: AsnHandlerTrait,
{
    fn run(&mut self, h: H) -> AsnError;
}
