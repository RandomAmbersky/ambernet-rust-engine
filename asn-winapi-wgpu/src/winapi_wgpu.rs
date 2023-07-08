use crate::wgpu_context::{new_context, AsnWgpuContext};
use winit::window::Window;

#[derive(Debug)]
pub struct AsnWinapiWgpu {
    ctx: AsnWgpuContext,
}

pub fn new(window: &Window) -> AsnWinapiWgpu {
    let ctx = new_context(window);
    AsnWinapiWgpu { ctx }
}
