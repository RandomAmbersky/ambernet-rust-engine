use winit::event::Event;
use winit::event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget};
use winit::window::{Window, WindowBuilder};

pub struct AsnEngine {
	window: Window
}

impl AsnEngine {
	pub fn new(event_loop: &EventLoop<()>) -> Self {
		let window = WindowBuilder::new().build(&event_loop).unwrap();
		AsnEngine {
			window
		}
	}

	pub fn process_event(&mut self, event: &Event<()>, event_loop_window_target: &EventLoopWindowTarget<()>, control_flow: &mut ControlFlow) {
		match event {
		}
	}
}
