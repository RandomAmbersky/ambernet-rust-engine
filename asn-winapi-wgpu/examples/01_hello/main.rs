use winit::event_loop::EventLoop;
use winit::window::{Window, WindowBuilder};

fn init() -> Window {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    window
}

pub fn main() {
    let window = init();
    let win_api = asn_winapi_wgpu::new(&window);
    println!("{:#?}", win_api);
}
