use asn_runner_winit;

use asn_core::{AsnError, AsnEvent, AsnHandlerTrait, AsnRunnerTrait};

struct MyHandler();

impl AsnHandlerTrait for MyHandler {
    fn proceed(&mut self, evt: &AsnEvent) -> Option<AsnError> {
        println!("{:?}", evt);
        Some(AsnError::Empty)
    }
}

fn main() {
    let mut runner = asn_runner_winit::new();
    let handler = MyHandler {};
    runner.run(handler);
}
