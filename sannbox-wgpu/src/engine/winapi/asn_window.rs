use crate::engine::winapi::event_converter::CustomEvent;
use asn_core::math::Size2D;
use wgpu::{Adapter, Device, Instance, Surface, SurfaceConfiguration};
use winit::dpi::PhysicalSize;
use winit::event_loop::EventLoop;
use winit::window::{Window, WindowBuilder};

pub struct AsnWindow {
    window: Window,
    surface: Surface,
    size: Size2D<u32>,
}

// getters
impl AsnWindow {
    pub fn get_current_texture(&self) -> wgpu::SurfaceTexture {
        self.surface
            .get_current_texture()
            .expect("Failed to get_current_texture")
    }
    pub fn get_adapter(&self, instance: &Instance) -> Adapter {
        instance
            .enumerate_adapters(wgpu::Backends::all())
            .find(|adapter| adapter.is_surface_supported(&self.surface))
            .unwrap()
    }
    fn get_config(&self, adapter: &Adapter, size: &PhysicalSize<u32>) -> SurfaceConfiguration {
        let surface_caps = self.surface.get_capabilities(adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        println!("get_config: {:?} ", surface_format);

        let caps_info = surface_caps.formats;
        println!("caps_info formats: {:?} ", caps_info);

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
    pub fn new(
        event_loop: &EventLoop<CustomEvent>,
        instance: &Instance,
        size: &Size2D<u32>,
    ) -> Self {
        let window = WindowBuilder::new().build(event_loop).unwrap();

        #[cfg(target_arch = "wasm32")]
        {
            // Winit prevents sizing with CSS, so we have to set
            // the size manually when on web.
            // use winit::dpi::PhysicalSize;
            // window.set_inner_size(PhysicalSize::new(size.width, size.height));

            use winit::platform::web::WindowExtWebSys;
            web_sys::window()
                .and_then(|win| win.document())
                .and_then(|doc| {
                    let dst = doc.get_element_by_id("wasm-example")?;
                    let canvas = web_sys::Element::from(window.canvas());
                    dst.append_child(&canvas).ok()?;
                    Some(())
                })
                .expect("Couldn't append canvas to document body.");
        }

        let surface = unsafe { instance.create_surface(&window).unwrap() };

        // let size = Size2D {
        //     width: 1024_u32,
        //     height: 768_u32,
        // };

        // let surface_caps = surface.get_capabilities(&adapter);
        // Shader code in this tutorial assumes an Srgb surface texture. Using a different
        // one will result all the colors comming out darker. If you want to support non
        // Srgb surfaces, you'll need to account for that when drawing to the frame.
        // let surface_format = surface_caps
        //     .formats
        //     .iter()
        //     .copied()
        //     .find(|f| f.is_srgb())
        //     .unwrap_or(surface_caps.formats[0]);

        // let config = wgpu::SurfaceConfiguration {
        //     usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        //     format: surface_format,
        //     width: size.width,
        //     height: size.height,
        //     present_mode: surface_caps.present_modes[0],
        //     alpha_mode: surface_caps.alpha_modes[0],
        //     view_formats: vec![],
        // };
        // surface.configure(&device, &config);

        AsnWindow {
            window,
            surface,
            size: *size,
        }
    }
    pub fn get_size(&self) -> Size2D<u32> {
        self.size
    }
    pub fn get_window(&self) -> &Window {
        &self.window
    }
    pub fn resize(&mut self, size: &Size2D<u32>) {
        self.size = *size;
    }
}

impl AsnWindow {
    pub fn configure_surface(&self, adapter: &Adapter, device: &Device) {
        let size = self.window.inner_size();
        let config = self.get_config(adapter, &size);
        self.surface.configure(device, &config);
    }
    pub fn request_redraw(&self) {
        self.window.request_redraw();
    }
}
