use asn_logger::trace;

use asn_core::traits::TAsnBaseEngine;
use winit::event::Event;
use winit::event_loop::{EventLoop, EventLoopWindowTarget};
use winit::window::WindowBuilder;

pub struct Engine {
    is_need_exit: bool,
}

impl TAsnBaseEngine for Engine {
    fn is_need_exit(&self) -> bool {
        self.is_need_exit
    }

    fn set_need_exit(&mut self) {
        self.is_need_exit = true
    }
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

        event_loop
            .run(|e, t| {
                // trace!("custom_event_handler: {:?} {:?}", evt, a);
                custom_event_handler(self, e, t)
            })
            .unwrap();
    }
}

fn custom_event_handler(e: &mut Engine, evt: Event<()>, t: &EventLoopWindowTarget<()>) {
    trace!("custom_event_handler: {:?} {:?}", evt, t);
    e.set_need_exit();
}
