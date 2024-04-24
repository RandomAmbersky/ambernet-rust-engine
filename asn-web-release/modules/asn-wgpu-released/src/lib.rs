use asn_core::traits::TAsnBaseEngine;
use asn_logger::trace;
use winit::event::Event;
use winit::event_loop::{EventLoop, EventLoopWindowTarget};
use winit::window::WindowBuilder;

pub fn run_loop<E>(e: &mut E)
where
    E: TAsnBaseEngine,
{
    trace!("Engine:run");
    let event_loop = EventLoop::new().unwrap();
    let _window = WindowBuilder::new().build(&event_loop).unwrap();

    event_loop
        .run(|evt, t| custom_event_handler(e, evt, t))
        .unwrap();
}

fn custom_event_handler<E>(e: &mut E, evt: Event<()>, t: &EventLoopWindowTarget<()>)
where
    E: TAsnBaseEngine,
{
    if e.is_need_exit() {
        t.exit();
        return;
    }
    trace!("custom_event_handler: {:?} {:?}", evt, t);
    // e.set_need_exit();
}
