use asn_logger::trace;

use winit::event::Event;
use winit::event_loop::{EventLoop, EventLoopWindowTarget};
use winit::window::WindowBuilder;

pub struct Engine {
    is_need_exit: bool,
}

impl Engine {
    pub fn new() -> Self {
        trace!("Engine:new");
        Engine {
            is_need_exit: false,
        }
    }
    pub fn init(&mut self) {
        trace!("Engine:init")
    }
    pub fn run(&mut self) {
        trace!("Engine:run");
        let event_loop = EventLoop::new().unwrap();
        let window = WindowBuilder::new().build(&event_loop).unwrap();

        let event_handler = custom_event_handler;
        event_loop.run(event_handler).unwrap();
    }
}

fn custom_event_handler(e: Event<()>, a: &EventLoopWindowTarget<()>) {
    trace!("custom_event_handler: {:?} {:?}", e, a);
}
