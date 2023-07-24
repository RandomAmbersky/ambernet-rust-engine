mod event_converter;
mod winapi;

use crate::engine::core::traits::{TAsnEngine, TAsnWinapi};
use crate::engine::event_runner::event_converter::convert_event;
use crate::engine::event_runner::winapi::AsnWgpuWinApi;
use winit::event_loop::{ControlFlow, EventLoop};

pub struct RunnerPreset {
    event_loop: Option<EventLoop<()>>,
}

pub type WinApi = AsnWgpuWinApi;
pub fn build() -> (RunnerPreset, WinApi) {
    let event_loop = EventLoop::new();

    let winapi = AsnWgpuWinApi::new(&event_loop);

    let preset = RunnerPreset {
        event_loop: Some(event_loop),
    };
    (preset, winapi)
}

pub fn run<E: 'static>(mut p: RunnerPreset, mut engine: E)
where
    E: TAsnEngine,
{
    let event_loop = p.event_loop.take().unwrap();
    event_loop.run(move |event, _event_loop_window_target, control_flow| {
        *control_flow = ControlFlow::Poll;

        let evt = convert_event(&event);
        if let Some(e) = evt {
            println!("event: {:?}", e);
            engine.get_winapi().redraw();
            // hanlder.proceed(&mut ctx, &e);
        }
    })
}
