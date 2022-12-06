mod window;

use crate::gfx::window::AsnWindow;
use wgpu::{Adapter, Device, Queue};
use winit::event_loop::EventLoop;

mod texture;
pub use texture::AsnTexture;

mod vertex;
pub use vertex::Vertex;

pub struct AsnGfx {
    pub main_window: AsnWindow,
    pub device: Device,
    pub adapter: Adapter,
    pub queue: Queue,
}

impl AsnGfx {
    pub fn new(event_loop: &EventLoop<()>) -> Self {
        let instance = wgpu::Instance::new(wgpu::Backends::all());

        let main_window = AsnWindow::new(event_loop, &instance);

        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            compatible_surface: Some(&main_window.surface),
            force_fallback_adapter: false,
            ..Default::default()
        }))
        .expect("Failed to find an appropriate adapter");

        let (device, queue) = pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::downlevel_defaults(),
            },
            None,
        ))
        .expect("Failed to create device");

        AsnGfx {
            main_window,
            device,
            adapter,
            queue,
        }
    }
}
