mod asn_window;
mod event_converter;

use crate::engine::event_runner::asn_window::AsnWindow;
use crate::engine::event_runner::event_converter::convert_event;
use winit::event_loop::{ControlFlow, EventLoop};

pub struct EventRunner {
    pub event_loop: Option<EventLoop<()>>,
    pub window: Option<AsnWindow>,
}

impl EventRunner {
    pub fn new() -> Self {
        let event_loop = EventLoop::new();
        let window = AsnWindow::new(&event_loop);
        EventRunner {
            event_loop: Some(event_loop),
            window: Some(window),
        }
    }
}

pub fn run(mut e: EventRunner) {
    let event_loop = e.event_loop.take().unwrap();

    event_loop.run(move |event, _event_loop_window_target, control_flow| {
        *control_flow = ControlFlow::Poll;

        let evt = convert_event(&event);
        if let Some(e) = evt {
            println!("event: {:?}", e);
            // hanlder.proceed(&mut ctx, &e);
        }
    })
}
