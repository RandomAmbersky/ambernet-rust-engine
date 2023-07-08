use crate::asn_window;
use winit::event_loop::EventLoop;

use crate::asn_window::AsnWindow;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

pub struct WinitContext {
    main_window: AsnWindow,
}

impl WinitContext {
    pub fn get_window(&self) -> &AsnWindow {
        &self.main_window
    }
}

pub fn new(event_loop: &EventLoop<()>) -> WinitContext {
    let main_window = asn_window::new(event_loop);
    WinitContext { main_window }
}
