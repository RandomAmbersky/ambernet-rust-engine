use log::{debug, error};
use winit::event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget};
use winit::window::{Window, WindowBuilder, WindowId};

pub struct AsnEngine {
    window: Window,
}

impl AsnEngine {
    pub fn new(event_loop: &EventLoop<()>) -> Self {
        let window = WindowBuilder::new().build(&event_loop).unwrap();
        AsnEngine { window }
    }

    pub fn process_event(
        &mut self,
        event: &Event<()>,
        event_loop_window_target: &EventLoopWindowTarget<()>,
        control_flow: &mut ControlFlow,
    ) {
        match event {
            Event::WindowEvent { window_id, event } => {
                return self.process_window_event(control_flow, window_id, event)
            }
            Event::RedrawRequested(window_id) => return self.process_redraw_requested(window_id),
            Event::MainEventsCleared {} => return self.process_main_events_cleared(),
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
            WindowEvent::CursorMoved { .. } => {}
            WindowEvent::CursorLeft { .. } => {}
            WindowEvent::CursorEntered { .. } => {}
            WindowEvent::Resized(size) => {
                debug!("resize window {:?}", size);
            }
            WindowEvent::KeyboardInput {
                device_id,
                input,
                is_synthetic,
            } => return self.process_keyboard_event(input, control_flow),
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
            return;
        }
    }
}

impl AsnEngine {
    fn process_main_events_cleared(&mut self) {
        self.window.request_redraw();
    }
}

impl AsnEngine {
    fn process_keyboard_event(&self, input: &KeyboardInput, control_flow: &mut ControlFlow) {
        match input {
            KeyboardInput {
                state: ElementState::Pressed,
                virtual_keycode: Some(VirtualKeyCode::Escape),
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => {}
        }
    }
}
