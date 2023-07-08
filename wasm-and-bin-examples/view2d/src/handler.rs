use asn_core::{AsnContext, AsnError, AsnEvent, AsnHandlerTrait, AsnWindowEvent};

pub struct MyHandler();

impl AsnHandlerTrait for MyHandler {
    fn proceed(&mut self, ctx: &mut AsnContext, evt: &AsnEvent) -> Option<AsnError> {
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
