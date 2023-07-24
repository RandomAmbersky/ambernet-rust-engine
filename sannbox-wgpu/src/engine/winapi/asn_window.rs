use crate::engine::core::math::Size2D;
use winit::event_loop::EventLoop;
use winit::window::{Window, WindowBuilder};

pub struct AsnWindow {
    window: Window,
    size: Size2D<u32>,
}

impl AsnWindow {
    pub fn new(event_loop: &EventLoop<()>) -> Self {
        let window = WindowBuilder::new().build(event_loop).unwrap();

        let size = Size2D {
            width: 1024_u32,
            height: 768_u32,
        };

        #[cfg(target_arch = "wasm32")]
        {
            // Winit prevents sizing with CSS, so we have to set
            // the size manually when on web.
            use winit::dpi::PhysicalSize;
            window.set_inner_size(PhysicalSize::new(size.width, size.height));

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
        AsnWindow { window, size }
    }
    pub fn get_size(&self) -> Size2D<u32> {
        self.size
    }
    pub fn get_window(&self) -> &Window {
        &self.window
    }
    pub fn resize(&mut self, size: Size2D<u32>) {
        self.size = size;
    }
}
