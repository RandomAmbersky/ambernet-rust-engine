use winit::event_loop::EventLoop;
use winit::window::{Window, WindowBuilder};

fn add_window(event_loop: &EventLoop<()>) -> Window {
    let window = WindowBuilder::new().build(event_loop).unwrap();
    window
}

pub fn main() {
    let event_loop = EventLoop::new();
    let window = add_window(&event_loop);
    let win_api = asn_winapi_wgpu::new(&window);
    println!("{:#?}", win_api);
}
