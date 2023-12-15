use crate::engine::winapi::event_converter::{convert_event, CustomEvent};
use crate::engine::winapi::wgpu::AsnWgpuWinApi;
use crate::engine::TAsnEngine;
use asn_core::traits::TAsnHandler;
use asn_logger::info;
use winit::error::EventLoopError;
use winit::event_loop::ControlFlow::{Poll, Wait};
use winit::event_loop::{EventLoop, EventLoopBuilder};

mod asn_window;
pub mod defines;
mod event_converter;
mod mesh;
mod resources;
mod scene;
mod utils;
mod wgpu;

pub type WinApi = AsnWgpuWinApi;

pub type AsnNodeQuad = scene::AsnWgpuNodeQuad;
pub type AsnNodeView2d = scene::AsnWgpuNodeView2d;

pub type AsnTexture = wgpu::texture::AsnTexture;

pub struct RunnerPreset {
    event_loop: Option<EventLoop<CustomEvent>>,
}

pub fn build() -> (RunnerPreset, WinApi) {
    // let event_loop = EventLoop::new();
    let event_loop = EventLoopBuilder::<CustomEvent>::with_user_event()
        .build()
        .unwrap();

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
) -> Result<(), EventLoopError> {
    let event_loop = p.event_loop.take().unwrap();
    event_loop.set_control_flow(Poll);

    event_loop.run(move |event, event_loop_window_target| {
        if eng.is_need_exit() {
            event_loop_window_target.exit();
            return;
        }

        let evt = convert_event(&event);
        if let Some(e) = evt {
            // info!("{:?}", event);
            h.handle(&e, &mut eng)
        }
    })
}
