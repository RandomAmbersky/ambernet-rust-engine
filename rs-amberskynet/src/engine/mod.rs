use log::{debug, error};
use winit::event::{Event, WindowEvent};
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
        // debug!("this is a event {:?}", event);
        match event {
            Event::WindowEvent { window_id, event } => {
                return self.process_window_event(control_flow, window_id, event)
            }
            Event::RedrawRequested(window_id) => return self.process_redraw_requested(window_id),
            Event::MainEventsCleared {} => return self.process_main_events_cleared(),
            _ => {}
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
        debug!("this is a debug {:?}", event);
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
