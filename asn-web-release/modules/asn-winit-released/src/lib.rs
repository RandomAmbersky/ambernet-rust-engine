mod event_converter;

use crate::event_converter::convert_event;
use asn_core::traits::{TAsnBaseEngine, TAsnHandler};
use asn_logger::trace;
use winit::event::Event;
use winit::event_loop::{EventLoop, EventLoopWindowTarget};
use winit::window::{Window, WindowBuilder};

struct RunnerDataset {
    main_window: Window,
}

pub fn run_loop<E, H>(e: &mut E, h: &mut H)
where
    E: TAsnBaseEngine,
    H: TAsnHandler<E>,
{
    trace!("Engine:run");
    let event_loop = EventLoop::new().unwrap();
    let main_window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut r = RunnerDataset { main_window };

    event_loop
        .run(|evt, t| event_handler(e, h, evt, t, &mut r))
        .unwrap();
}

fn event_handler<E, H>(
    engine: &mut E,
    h: &mut H,
    evt: Event<()>,
    t: &EventLoopWindowTarget<()>,
    r: &mut RunnerDataset,
) where
    E: TAsnBaseEngine,
    H: TAsnHandler<E>,
{
    if engine.is_need_exit() {
        t.exit();
        return;
    }
    match convert_event(&evt) {
        None => {}
        Some(e) => {
            h.handle(&e, engine)
            // trace!("AsnEvent: {:?}", e);
            // engine.handle(&e).unwrap();
        }
    }
    // e.set_need_exit();
    // trace!("custom_event_handler: {:?} {:?}", evt, t);
}
