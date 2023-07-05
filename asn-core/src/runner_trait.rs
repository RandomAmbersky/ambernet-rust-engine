use crate::asn_errors::AsnError;

pub trait AsnRunnerTrait {
    fn run() -> AsnError;
    fn set_handler();
}
