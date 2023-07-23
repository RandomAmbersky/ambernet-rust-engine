mod event_converter;
mod winapi;

use crate::engine::event_runner::event_converter::convert_event;
use crate::engine::event_runner::winapi::AsnWgpuWinApi;
use winit::event_loop::{ControlFlow, EventLoop};

pub struct EventRunner {
    pub event_loop: EventLoop<()>,
    pub winapi: AsnWgpuWinApi,
}

impl EventRunner {
    pub fn new() -> Self {
        let event_loop = EventLoop::new();
        let winapi = AsnWgpuWinApi::new(&event_loop);
        EventRunner { event_loop, winapi }
    }
}

pub fn run(e: EventRunner) {
    e.event_loop
        .run(move |event, _event_loop_window_target, control_flow| {
            *control_flow = ControlFlow::Poll;

            let evt = convert_event(&event);
            if let Some(e) = evt {
                println!("event: {:?}", e);
                // hanlder.proceed(&mut ctx, &e);
            }
        })
}
