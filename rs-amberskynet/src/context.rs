use crate::gfx::AsnGfx;
use winit::event_loop::EventLoop;

pub struct AsnContext {
    pub gfx: AsnGfx,
}

pub fn new(event_loop: &EventLoop<()>) -> AsnContext {
    let gfx = AsnGfx::new(event_loop);
    AsnContext { gfx }
}
