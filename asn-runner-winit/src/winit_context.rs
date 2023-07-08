use winit::event_loop::EventLoop;
use winit::window::{Window, WindowBuilder};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

pub struct WinitContext {
    main_window: Window,
}

impl WinitContext {
    pub fn get_window(&self) -> &Window {
        &self.main_window
    }
}

pub fn new(event_loop: &EventLoop<()>) -> WinitContext {
    let window = WindowBuilder::new().build(event_loop).unwrap();

    #[cfg(target_arch = "wasm32")]
    {
        // Winit prevents sizing with CSS, so we have to set
        // the size manually when on web.
        use winit::dpi::PhysicalSize;
        window.set_inner_size(PhysicalSize::new(450, 400));

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

    WinitContext {
        main_window: window,
    }
}
