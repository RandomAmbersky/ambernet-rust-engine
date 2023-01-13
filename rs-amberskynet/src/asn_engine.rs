use crate::context;
use crate::context::AsnContext;
use winit::event_loop::EventLoop;

pub fn init() -> (AsnContext, EventLoop<()>) {
    let event_loop = EventLoop::new();
    let ctx = context::new(&event_loop);
    (ctx, event_loop)
}
