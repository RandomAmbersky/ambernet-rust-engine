use crate::library::Context;
use asn_core::{AsnError, AsnEvent, AsnHandlerTrait, AsnWinapiTrait, AsnWindowEvent};
use asn_logger::info;
use asn_runner_winit::AsnScene;
use asn_scenegraph_core::AsnScenegraphTrait;

pub struct Handler {}

impl Handler {
    pub fn new() -> Self {
        Handler {}
    }
}

impl AsnHandlerTrait<Context> for Handler {
    fn proceed(&mut self, ctx: &mut Context, evt: &AsnEvent) -> Option<AsnError> {
        info!("{:?}", evt);
        match evt {
            AsnEvent::Empty => None,
            AsnEvent::WindowEvent(w) => match w {
                AsnWindowEvent::Resized(size) => {
                    ctx.get_winapi().window_resize(size);
                    None
                }
                AsnWindowEvent::RedrawRequested => None,
                AsnWindowEvent::CloseRequested => {
                    ctx.set_need_exit();
                    None
                }
            },
        }
    }
}
