mod viewport;
mod viewport_desc;
mod asn_engine_state;

use log::{debug, error};
use winit::event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget};
use winit::window::{Window, WindowBuilder, WindowId};
use crate::engine::asn_engine_state::AsnEngineState;
use crate::engine::viewport_desc::ViewportDesc;
use crate::state::State;

pub struct AsnEngine {
    window: Window,
}

impl AsnEngine {
    pub fn new(event_loop: &EventLoop<()>) -> Self {

        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let window = WindowBuilder::new().build(&event_loop).unwrap();

        let window_color = wgpu::Color {
            r: 0.5,
            g: 0.5,
            b: 0.5,
            a: 1.0,
        };

        let viewport_desc = ViewportDesc::new(window, window_color, &instance);

        let adapter = instance
          .request_adapter(&wgpu::RequestAdapterOptions {
              // Request an adapter which can render to our surface
              compatible_surface: Option::from(&viewport_desc.surface),
              ..Default::default()
          })
          .await
          .expect("Failed to find an appropriate adapter");

        AsnEngine { window }
    }

    pub fn process_event(
        &mut self,
        event: &Event<()>,
        _event_loop_window_target: &EventLoopWindowTarget<()>,
        control_flow: &mut ControlFlow,
    ) {
        match event {
            Event::WindowEvent { window_id, event } => {
                self.process_window_event(control_flow, window_id, event)
            }
            Event::RedrawRequested(window_id) => self.process_redraw_requested(window_id),
            Event::MainEventsCleared {} => self.process_main_events_cleared(),
            Event::RedrawEventsCleared {} => {}
            Event::NewEvents(..) => {}
            _ => {
                // debug!("unprocesseble event {:?}", event);
            }
        }
    }
}

impl AsnEngine {
    fn process_window_event(
        &mut self,
        control_flow: &mut ControlFlow,
        window_id: &WindowId,
        event: &WindowEvent,
    ) {
        if window_id != &self.window.id() {
            error!("not correct window_id: {:?}", window_id);
            return;
        }
        match event {
            WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
            WindowEvent::ScaleFactorChanged { .. } => {
                // TODO state.resize(**new_inner_size);
            }
            WindowEvent::CursorMoved { .. } => {}
            WindowEvent::CursorLeft { .. } => {}
            WindowEvent::CursorEntered { .. } => {}
            WindowEvent::Resized(size) => {
                // TODO state.resize(*physical_size);
                debug!("resize window {:?}", size);
            }
            WindowEvent::KeyboardInput {
                device_id: _,
                input,
                is_synthetic: _,
            } => self.process_keyboard_event(input, control_flow),
            _ => {
                debug!("unprocesseble window event {:?}", event);
            }
        }
        // debug!("this is a debug {:?}", event);
    }
}

impl AsnEngine {
    fn process_redraw_requested(&mut self, window_id: &WindowId) {
        if window_id != &self.window.id() {
            error!("not correct window_id: {:?}", window_id);
        }
        // TODO state.update();
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
    }
}

impl AsnEngine {
    fn process_main_events_cleared(&mut self) {
        self.window.request_redraw();
    }
}

impl AsnEngine {
    fn process_keyboard_event(&self, input: &KeyboardInput, control_flow: &mut ControlFlow) {
        if let KeyboardInput {
            state: ElementState::Pressed,
            virtual_keycode: Some(VirtualKeyCode::Escape),
            ..
        } = input
        {
            *control_flow = ControlFlow::Exit
        }
    }
}
