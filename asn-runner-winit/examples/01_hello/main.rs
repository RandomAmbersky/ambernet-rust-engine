use asn_core::{AsnContext, AsnError, AsnEvent, AsnHandlerTrait};

struct MyHandler();

impl AsnHandlerTrait for MyHandler {
    fn proceed(&mut self, ctx: &mut AsnContext, evt: &AsnEvent) -> Option<AsnError> {
        println!("{:?}", evt);
        ctx.is_need_exit = true;
        Some(AsnError::Empty)
    }
}

fn main() {
    let h = MyHandler {};
    asn_runner_winit::run(h);
}
