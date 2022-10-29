mod vertex;
mod texture;
mod state;
mod view_2d;
mod renderer;
mod engine;

use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use vertex::Vertex;
use view_2d::resource::{INDICES, VERTICES};
use crate::engine::AsnEngine;
use crate::renderer::AsnRenderer;

use crate::state::State;

pub async fn run() {
    env_logger::init();

    let event_loop = EventLoop::new();

    let mut engine = AsnEngine::new(&event_loop);

    event_loop.run(move |event, event_loop_window_target, control_flow| {
        engine.process_event(&event, event_loop_window_target, control_flow)
    })

    // // State::new uses async code, so we're going to wait for it to finish
    // let mut state = State::new(&window).await;

    // event_loop.run(move |event, _, control_flow| {
    //     renderer.process_event(&event, control_flow)
        // match event {
        //     Event::WindowEvent {
        //         ref event,
        //         window_id,
        //     } if window_id == window.id() => {
        //         if !state.input(event) {
        //             match event {
        //                 WindowEvent::CloseRequested
        //                 | WindowEvent::KeyboardInput {
        //                     input:
        //                         KeyboardInput {
        //                             state: ElementState::Pressed,
        //                             virtual_keycode: Some(VirtualKeyCode::Escape),
        //                             ..
        //                         },
        //                     ..
        //                 } => *control_flow = ControlFlow::Exit,
        //                 WindowEvent::Resized(physical_size) => {
        //                     state.resize(*physical_size);
        //                 }
        //                 WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
        //                     // new_inner_size is &mut so w have to dereference it twice
        //                     state.resize(**new_inner_size);
        //                 }
        //                 _ => {}
        //             }
        //         }
        //     }
        //     Event::RedrawRequested(window_id) if window_id == window.id() => {
        //         state.update();
        //         match state.render() {
        //             Ok(_) => {}
        //             // Reconfigure the surface if it's lost or outdated
        //             Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
        //                 state.reload_size()
        //             }
        //             // The system is out of memory, we should probably quit
        //             Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
        //             // We're ignoring timeouts
        //             Err(wgpu::SurfaceError::Timeout) => log::warn!("Surface timeout"),
        //         }
        //     }
        //     Event::MainEventsCleared => {
        //         // RedrawRequested will only trigger once, unless we manually
        //         // request it.
        //         window.request_redraw();
        //     }
        //     _ => {}
        // }
    // });
}
