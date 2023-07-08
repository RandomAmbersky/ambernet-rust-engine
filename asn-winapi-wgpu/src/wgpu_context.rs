use wgpu::{InstanceDescriptor, Surface};
use winit::window::Window;

#[derive(Debug)]
pub struct AsnWgpuContext {
    surface: Surface,
}

pub fn new_context(window: &Window) -> AsnWgpuContext {
    let instance = wgpu::Instance::new(InstanceDescriptor {
        backends: wgpu::Backends::all(),
        dx12_shader_compiler: Default::default(),
    });

    let surface = unsafe { instance.create_surface(&window).unwrap() };

    AsnWgpuContext { surface }
}
