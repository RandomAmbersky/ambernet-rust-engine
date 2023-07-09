use crate::my_context::MyCtx;
use asn_core::{AsnContextTrait, AsnError, AsnEvent, AsnHandlerTrait, AsnWindowEvent};
use asn_runner_winit::AsnWgpuWinApi;

pub struct MyHandler {}

impl AsnHandlerTrait<AsnWgpuWinApi, MyCtx> for MyHandler {
    fn proceed(&mut self, ctx: &mut MyCtx, evt: &AsnEvent) -> Option<AsnError> {
        println!("{:?}", evt);
        match evt {
            AsnEvent::WindowEvent(e) => match e {
                AsnWindowEvent::Resized(_) => None,
                AsnWindowEvent::RedrawRequested => None,
                AsnWindowEvent::CloseRequested => {
                    ctx.set_need_exit();
                    None
                }
            },
            _ => None,
        }
    }
}
