mod event_converter;

use crate::event_converter::convert_event;
use asn_core::traits::{TAsnBaseEngine, TAsnHandleEngine};
use asn_logger::trace;
use winit::event::Event;
use winit::event_loop::{EventLoop, EventLoopWindowTarget};
use winit::window::{Window, WindowBuilder};

struct RunnerDataset {
    main_window: Window,
}

pub fn run_loop<E>(e: &mut E)
where
    E: TAsnBaseEngine + TAsnHandleEngine,
{
    trace!("Engine:run");
    let event_loop = EventLoop::new().unwrap();
    let main_window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut r = RunnerDataset { main_window };

    event_loop
        .run(|evt, t| event_handler(e, evt, t, &mut r))
        .unwrap();
}

fn event_handler<E>(
    engine: &mut E,
    evt: Event<()>,
    t: &EventLoopWindowTarget<()>,
    r: &mut RunnerDataset,
) where
    E: TAsnBaseEngine + TAsnHandleEngine,
{
    if engine.is_need_exit() {
        t.exit();
        return;
    }
    match convert_event(&evt) {
        None => {}
        Some(e) => {
            trace!("AsnEvent: {:?}", e);
            engine.handle(&e).unwrap();
        }
    }
    // e.set_need_exit();
    // trace!("custom_event_handler: {:?} {:?}", evt, t);
}
