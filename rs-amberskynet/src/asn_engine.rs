use crate::context::AsnContext;
use crate::{context, ExtHandlerTrait};
use winit::event::Event;
use winit::event_loop::{ControlFlow, EventLoop};

pub fn init() -> (AsnContext, EventLoop<()>) {
    let event_loop = EventLoop::new();
    let ctx = context::new(&event_loop);
    (ctx, event_loop)
}

pub fn process_event(_ctx: &AsnContext, _event: &Event<()>, _control_flow: &mut ControlFlow) {}
