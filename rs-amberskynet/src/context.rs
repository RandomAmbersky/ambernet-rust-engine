use rs_gfx_wgpu::AsnGfx;
use winit::event_loop::EventLoop;

pub struct AsnContext {
    pub gfx: AsnGfx,
    pub is_need_exit: bool,
}

pub fn new(event_loop: &EventLoop<()>) -> AsnContext {
    let gfx = AsnGfx::new(event_loop);
    AsnContext {
        gfx,
        is_need_exit: false,
    }
}
