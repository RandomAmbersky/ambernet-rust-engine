use winit::event::Event;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

mod window;

pub struct AsnRenderer {
	window: winit::window::Window
}

impl AsnRenderer {
	pub fn new(event_loop: &EventLoop<()>) -> Self {
		let window = WindowBuilder::new().build(&event_loop).unwrap();

		AsnRenderer {
			window
		}
	}
	pub fn process_event(&mut self, event: &Event<()>, control_flow: &mut ControlFlow) {

	}
}
