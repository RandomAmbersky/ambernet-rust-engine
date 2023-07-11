use crate::library::Context;
use asn_core::{AsnError, AsnEvent, AsnHandlerTrait, AsnWindowEvent};
use asn_logger::info;
use winit::event::VirtualKeyCode::N;

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
    fn proceed(&mut self, mut ctx: &mut Context, evt: &AsnEvent) -> Option<AsnError> {
        info!("{:?}", evt);
        match evt {
            AsnEvent::Empty => None,
            AsnEvent::WindowEvent(w) => match w {
                AsnWindowEvent::Resized(_) => None,
                AsnWindowEvent::RedrawRequested => None,
                AsnWindowEvent::CloseRequested => {
                    ctx.set_need_exit();
                    None
                }
            },
        }
    }
}
