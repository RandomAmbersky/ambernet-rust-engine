use asn_core::AsnContext;
use winit::event_loop::EventLoop;
use winit::window::{Window, WindowBuilder};

pub struct WinitContext {
    pub main_window: Window,
    pub ctx: AsnContext,
}

impl WinitContext {
    pub fn new(event_loop: &EventLoop<()>) -> Self {
        let ctx = AsnContext::default();
        let main_window = WindowBuilder::new().build(&event_loop).unwrap();
        Self { ctx, main_window }
    }
}
