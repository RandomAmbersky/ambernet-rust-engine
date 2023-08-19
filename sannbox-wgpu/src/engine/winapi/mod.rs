use crate::engine::core::traits::TAsnHandler;
use crate::engine::winapi::event_converter::{convert_event, CustomEvent};
use crate::engine::winapi::scene::{AsnWgpuNodeQuad, AsnWgpuNodeView2d};
use crate::engine::winapi::wgpu::AsnWgpuWinApi;
use crate::engine::TAsnEngine;
use winit::event_loop::{ControlFlow, EventLoop, EventLoopBuilder, EventLoopProxy};

mod asn_window;
pub mod defines;
mod event_converter;
mod resources;
mod scene;
mod utils;
mod wgpu;

pub type WinApi = AsnWgpuWinApi;
pub type NodeQuad = AsnWgpuNodeQuad;
pub type NodeView2d = AsnWgpuNodeView2d;
pub type AsnTexture = wgpu::texture::AsnTexture;

pub struct RunnerPreset {
    event_loop: Option<EventLoop<CustomEvent>>,
}

pub fn build() -> (RunnerPreset, WinApi) {
    // let event_loop = EventLoop::new();
    let event_loop = EventLoopBuilder::<CustomEvent>::with_user_event().build();

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
            h.handle(&e, &mut eng)
        }
    })
}
