mod asn_window;
mod wgpu_context;

use crate::asn_winapi::asn_window::AsnWindow;
use crate::asn_winapi::wgpu_context::AsnWgpuContext;
use asn_core::AsnEvent;
use winit::event::Event;
use winit::event_loop::EventLoop;

pub struct AsnWgpuWinApi {
    window: AsnWindow,
    wgpu_ctx: AsnWgpuContext,
}

pub fn new(event_loop: &EventLoop<()>) -> AsnWgpuWinApi {
    let window = asn_window::new(event_loop);
    let wgpu_ctx = wgpu_context::new(&window.get_window());
    AsnWgpuWinApi { window, wgpu_ctx }
}

impl AsnWgpuWinApi {
    pub fn proceed(&mut self, e: &AsnEvent) {}
}
