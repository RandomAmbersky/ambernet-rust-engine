use crate::library::Context;
use asn_core::{AsnError, AsnEvent, AsnHandlerTrait, AsnWinapiTrait, AsnWindowEvent};
use asn_logger::info;

pub struct Handler {}

impl Handler {
    pub fn new() -> Self {
        Handler {}
    }
}

pub fn get_handler() -> Handler {
    Handler::new()
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
