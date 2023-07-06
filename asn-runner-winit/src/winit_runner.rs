use asn_core::{AsnError, AsnEvent, AsnHandlerTrait, AsnRunnerTrait};

#[derive(Default)]
pub struct WinitRunner {}

impl WinitRunner {
    pub fn new() -> Self {
        WinitRunner {}
    }
}

impl<H> AsnRunnerTrait<H> for WinitRunner
where
    H: AsnHandlerTrait,
{
    fn run(&mut self, mut h: H) -> AsnError {
        let evt = AsnEvent::Empty;
        let err = h.proceed(&evt);
        if let Some(..) = err {
            return err.unwrap();
        }
        AsnError::Empty
    }
}
