use crate::context::AsnContext;
use crate::{context, ExtHandlerTrait};
use winit::event::Event;
use winit::event_loop::{ControlFlow, EventLoop};

pub fn init() -> (AsnContext, EventLoop<()>) {
    let ctx = context::new();
    let event_loop = EventLoop::new();
    (ctx, event_loop)
}

pub fn process_event(_ctx: &AsnContext, _event: &Event<()>, _control_flow: &mut ControlFlow) {}

pub fn run<E>(ctx: AsnContext, event_loop: EventLoop<()>, ext: E)
where
    E: ExtHandlerTrait,
{
    ext.draw(&ctx);
    ext.update(&ctx);
}
