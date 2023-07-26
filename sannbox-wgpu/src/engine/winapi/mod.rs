use crate::engine::core::traits::{TAsnHandler, TAsnWinapi};
use crate::engine::winapi::event_converter::convert_event;
use crate::engine::winapi::scene::AsnWgpuNodeQuad;
use crate::engine::winapi::wgpu::AsnWgpuWinApi;
use crate::engine::TAsnEngine;
use winit::event_loop::{ControlFlow, EventLoop};

mod asn_window;
mod event_converter;
mod resources;
mod scene;
mod utils;
mod wgpu;

pub type WinApi = AsnWgpuWinApi;
pub type NodeQuad = AsnWgpuNodeQuad;

pub struct RunnerPreset {
    event_loop: Option<EventLoop<()>>,
}

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
            h.handle(&e, &mut eng)
        }
    })
}
