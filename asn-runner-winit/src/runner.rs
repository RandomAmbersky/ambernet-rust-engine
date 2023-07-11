use crate::asn_winapi::AsnWgpuWinApi;
use winit::event_loop::EventLoop;

pub struct Runner {
    event_loop: EventLoop<()>,
}

impl Runner {
    pub fn new() -> Self {
        let event_loop = EventLoop::new();
        Runner { event_loop }
    }
    pub fn new_winapi(&self) -> AsnWgpuWinApi {
        let winapi = AsnWgpuWinApi::new(&self.event_loop);
        winapi
    }
}
