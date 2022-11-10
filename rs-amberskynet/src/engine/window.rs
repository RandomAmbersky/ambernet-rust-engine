use wgpu::{Adapter, Device, Instance, Surface, TextureFormat};
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
    pub fn get_supported_format(&self, adapter: &Adapter) -> TextureFormat {
        self.surface.get_supported_formats(adapter)[0]
    }
}

impl AsnWindow {
    pub fn new(event_loop: &EventLoop<()>, instance: &Instance) -> Self {
        let window = WindowBuilder::new().build(event_loop).unwrap();
        let surface = unsafe { instance.create_surface(&window) };
        Self { window, surface }
    }
    // pub fn init(&mut self, device: &Device, adapter: &Adapter) {
    //     let size = self.window.inner_size();
    //
    //     let config = wgpu::SurfaceConfiguration {
    //         usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
    //         format: self.surface.get_supported_formats(adapter)[0],
    //         width: size.width,
    //         height: size.height,
    //         present_mode: wgpu::PresentMode::Fifo,
    //         alpha_mode: self.surface.get_supported_alpha_modes(adapter)[0],
    //     };
    //     self.surface.configure(device, &config);
    //     // self.config = Some(config)
    // }
    pub fn resize(&self, device: &Device, size: winit::dpi::PhysicalSize<u32>) {
        // self.config.unwrap().width = size.width;
        // self.config.unwrap().height = size.height;
        // self.surface.configure(device, &self.config.unwrap());
    }
    pub fn reload_size(&self, device: &Device) {
        let size = self.window.inner_size();
        self.resize(device, size);
    }
    pub fn request_redraw(&self) {
        self.window.request_redraw();
    }
}
