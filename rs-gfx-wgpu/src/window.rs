use wgpu::{Adapter, Device, Instance, Surface, SurfaceConfiguration, TextureFormat};
use winit::dpi::PhysicalSize;
use winit::event_loop::EventLoop;
use winit::window::{Window, WindowBuilder, WindowId};

pub struct AsnWindow {
    pub window: Window,
    pub surface: Surface,
}

// getters
impl AsnWindow {
    pub fn get_windows_id(&self) -> WindowId {
        self.window.id()
    }
    pub fn get_current_texture(&self) -> wgpu::SurfaceTexture {
        self.surface
            .get_current_texture()
            .expect("Failed to get_current_texture")
    }
    pub fn get_format(&self, adapter: &Adapter) -> TextureFormat {
        let surface_caps = self.surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            // .filter(|f| f.is_srgb)
            .next()
            .unwrap_or(surface_caps.formats[0]);
        surface_format
    }
    pub fn get_config(&self, adapter: &Adapter, size: &PhysicalSize<u32>) -> SurfaceConfiguration {
        let surface_caps = self.surface.get_capabilities(adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            // .filter(|f| f.is_srgb)
            .next()
            .unwrap_or(surface_caps.formats[0]);

        SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        }
    }
}

impl AsnWindow {
    pub fn new(event_loop: &EventLoop<()>, instance: &Instance) -> Self {
        let window = WindowBuilder::new().build(event_loop).unwrap();
        let surface = unsafe { instance.create_surface(&window).unwrap() };
        Self { window, surface }
    }
    pub fn configure_surface(&self, adapter: &Adapter, device: &Device) {
        let size = self.window.inner_size();
        let config = self.get_config(adapter, &size);
        self.surface.configure(device, &config);
    }
    pub fn request_redraw(&self) {
        self.window.request_redraw();
    }
}
