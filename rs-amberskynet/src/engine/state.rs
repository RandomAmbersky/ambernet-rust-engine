use crate::engine::window::AsnWindow;
use wgpu::{Adapter, Device, Queue};
use winit::event_loop::EventLoop;

pub struct AsnState {
    pub main_window: AsnWindow,
    pub device: Device,
    pub adapter: Adapter,
    pub queue: Queue,
}

impl AsnState {
    pub async fn new(event_loop: &EventLoop<()>) -> Self {
        let instance = wgpu::Instance::new(wgpu::Backends::all());

        let mut main_window = AsnWindow::new(event_loop, &instance);

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                compatible_surface: Some(&main_window.surface),
                force_fallback_adapter: false,
                ..Default::default()
            })
            .await
            .expect("Failed to find an appropriate adapter");

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::downlevel_defaults(),
                },
                None,
            )
            .await
            .expect("Failed to create device");

        let size = main_window.window.inner_size();
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: main_window.surface.get_supported_formats(&adapter)[0],
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: main_window.surface.get_supported_alpha_modes(&adapter)[0],
        };
        main_window.surface.configure(&device, &config);

        AsnState {
            main_window,
            device,
            adapter,
            queue,
        }
    }
}
