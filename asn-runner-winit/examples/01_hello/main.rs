use asn_core::{AsnContext, AsnError, AsnEvent, AsnHandlerTrait, AsnWindowEvent};

struct MyHandler();

impl AsnHandlerTrait for MyHandler {
    fn proceed(&mut self, ctx: &mut AsnContext, evt: &AsnEvent) -> Option<AsnError> {
        println!("{:?}", evt);
        match evt {
            AsnEvent::WindowEvent {
                window_id: _,
                event,
            } => match event {
                AsnWindowEvent::Resized(_) => None,
                AsnWindowEvent::RedrawRequested => None,
                AsnWindowEvent::CloseRequested => {
                    ctx.is_need_exit = true;
                    None
                }
            },
            _ => None,
        }
    }
}

fn main() {
    let h = MyHandler {};
    asn_runner_winit::run(h);
}
