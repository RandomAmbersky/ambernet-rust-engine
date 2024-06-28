use std::fs::OpenOptions;
use std::thread;
use winit::application::ApplicationHandler;
use winit::event::{Event, WindowEvent};
use winit::event_loop;
use winit::event_loop::{ActiveEventLoop, EventLoop, EventLoopProxy};
use winit::window::{Window, WindowId};

struct App {
    window: Option<Window>,
    proxy_loop: Option<EventLoopProxy<Event<()>>>,
}

impl ApplicationHandler<Event<()>> for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // let window_attributes = Window::default_attributes().with_title("A fantastic window!");
        // self.window = Some(event_loop.create_window(window_attributes).unwrap());
    }
    fn user_event(&mut self, event_loop: &ActiveEventLoop, event: Event<()>) {
        println!("User event: {:?}", event);
        event_loop::dispatch_event_for_app(self, event_loop, event);
    }
    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        // println!("window_event {:?}", event);
    }
    fn exiting(&mut self, event_loop: &ActiveEventLoop) {
        println!("exiting");
    }
    fn memory_warning(&mut self, event_loop: &ActiveEventLoop) {
        println!("memory_warning");
    }
}

fn main() {
    let event_loop = EventLoop::<Event<()>>::with_user_event().build().unwrap();

    let event_loop_proxy: EventLoopProxy<Event<()>> = event_loop.create_proxy();
    let mut app = App {
        window: None,
        proxy_loop: None,
    };

    thread::spawn(move || {
        for i in 0..10 {
            let e: Event<()> = Event::MemoryWarning;
            event_loop_proxy.send_event(e.clone()).unwrap();
            event_loop_proxy.send_event(e.clone()).unwrap();
            event_loop_proxy.send_event(e.clone()).unwrap();
            event_loop_proxy.send_event(e.clone()).unwrap();
        }
    });
    event_loop.run_app(&mut app).unwrap();
}
