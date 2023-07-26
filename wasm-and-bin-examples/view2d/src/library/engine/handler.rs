use asn_core::{
    AsnBaseContextTrait, AsnContext, AsnError, AsnEvent, AsnHandlerTrait, AsnWindowEvent,
};

pub struct Handler {}

impl Handler {
    pub fn new() -> Self {
        Self {}
    }
}

impl AsnHandlerTrait for Handler {
    fn proceed(&mut self, ctx: &mut AsnContext, evt: &AsnEvent) -> Option<AsnError> {
        println!("{:?}", evt);
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
