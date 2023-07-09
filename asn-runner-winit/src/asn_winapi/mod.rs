mod asn_window;

use crate::asn_winapi::asn_window::AsnWindow;
use asn_core::{AsnEvent, AsnWinapiTrait, Size2D};
use wgpu::{Adapter, Device, InstanceDescriptor, Queue, Surface};
use winit::event_loop::EventLoop;

pub struct AsnWgpuWinApi {
    window: AsnWindow,
    surface: Surface,
    config: wgpu::SurfaceConfiguration,
    adapter: Adapter,
    device: Device,
    queue: Queue,
}

pub fn new(event_loop: &EventLoop<()>) -> AsnWgpuWinApi {
    let window = asn_window::new(event_loop);

    let instance = wgpu::Instance::new(InstanceDescriptor {
        backends: wgpu::Backends::all(),
        dx12_shader_compiler: Default::default(),
    });

    let surface = unsafe { instance.create_surface(&window.get_window()).unwrap() };

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

    let surface_caps = surface.get_capabilities(&adapter);
    // Shader code in this tutorial assumes an Srgb surface texture. Using a different
    // one will result all the colors comming out darker. If you want to support non
    // Srgb surfaces, you'll need to account for that when drawing to the frame.
    let surface_format = surface_caps
        .formats
        .iter()
        .copied()
        .find(|f| f.is_srgb())
        .unwrap_or(surface_caps.formats[0]);
    let config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: surface_format,
        width: window.get_size().width,
        height: window.get_size().height,
        present_mode: surface_caps.present_modes[0],
        alpha_mode: surface_caps.alpha_modes[0],
        view_formats: vec![],
    };
    surface.configure(&device, &config);

    AsnWgpuWinApi {
        window,
        surface,
        config,
        adapter,
        device,
        queue,
    }
}

impl AsnWinapiTrait for AsnWgpuWinApi {
    fn window_resize(&mut self, new_size: Size2D<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
            self.surface.configure(&self.device, &self.config);
        }
    }
}
