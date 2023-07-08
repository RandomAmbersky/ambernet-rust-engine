use wgpu::{Adapter, Device, InstanceDescriptor, Queue, Surface};
use winit::window::Window;

#[derive(Debug)]
pub struct AsnWgpuContext {
    surface: Surface,
    adapter: Adapter,
    device: Device,
    queue: Queue,
}

pub fn new_context(window: &Window) -> AsnWgpuContext {
    let instance = wgpu::Instance::new(InstanceDescriptor {
        backends: wgpu::Backends::all(),
        dx12_shader_compiler: Default::default(),
    });

    let surface = unsafe { instance.create_surface(&window).unwrap() };

    let adapter = instance
        .enumerate_adapters(wgpu::Backends::all())
        .find(|adapter| adapter.is_surface_supported(&surface))
        .unwrap();

    let (device, queue) = pollster::block_on(adapter.request_device(
        &wgpu::DeviceDescriptor {
            features: wgpu::Features::empty(),
            // WebGL doesn't support all of wgpu's features, so if
            // we're building for the web we'll have to disable some.
            limits: if cfg!(target_arch = "wasm32") {
                wgpu::Limits::downlevel_webgl2_defaults()
            } else {
                wgpu::Limits::default()
            },
            label: None,
        },
        None, // Trace path
    ))
    .unwrap();

    AsnWgpuContext {
        surface,
        adapter,
        device,
        queue,
    }
}
