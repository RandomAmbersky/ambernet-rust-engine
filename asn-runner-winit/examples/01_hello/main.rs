use asn_core::{AsnContextTrait, AsnError, AsnEvent, AsnHandlerTrait, AsnWindowEvent};

struct MyCtx {
    is_need_exit: bool,
}

impl AsnContextTrait for MyCtx {
    fn is_need_exit(&self) -> bool {
        self.is_need_exit
    }

    fn set_need_exit(&mut self) {
        self.is_need_exit = true
    }
}

struct MyHandler {}

impl AsnHandlerTrait<MyCtx> for MyHandler {
    fn proceed(&mut self, ctx: &mut MyCtx, evt: &AsnEvent) -> Option<AsnError> {
        println!("{:?}", evt);
        match evt {
            AsnEvent::WindowEvent(e) => match e {
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
    let ctx = MyCtx {
        is_need_exit: false,
    };
    let h = MyHandler {};
    asn_runner_winit::run(ctx, h);
}
