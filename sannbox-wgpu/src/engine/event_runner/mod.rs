mod event_converter;
mod winapi;

use crate::engine::core::events::{AsnEvent, AsnWindowEvent};
use crate::engine::core::traits::{TAsnEngine, TAsnHandler, TAsnWinapi};
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

pub fn run<E: 'static + TAsnEngine, H: 'static + TAsnHandler<E>>(
    mut p: RunnerPreset,
    mut eng: E,
    mut h: H,
) {
    let event_loop = p.event_loop.take().unwrap();
    event_loop.run(move |event, _event_loop_window_target, control_flow| {
        *control_flow = ControlFlow::Poll;

        if eng.is_need_exit() {
            *control_flow = ControlFlow::Exit;
            return;
        }

        let evt = convert_event(&event);
        if let Some(e) = evt {
            println!("event: {:?}", e);
            handle_default(&e, &mut eng);
            h.handle(&e, &mut eng)
        }
    })
}

fn handle_default<E: TAsnEngine>(evt: &AsnEvent, e: &mut E) {
    println!("base handle {:?} event", &evt);
    match evt {
        AsnEvent::Empty => {}
        AsnEvent::WindowEvent(w) => match w {
            AsnWindowEvent::Resized(size) => {
                e.get_winapi().window_resize(size);
                e.get_winapi().redraw();
            }
            AsnWindowEvent::RedrawRequested => {
                e.get_winapi().redraw();
            }
            AsnWindowEvent::CloseRequested => {
                e.set_need_exit();
            }
        },
    }
}
