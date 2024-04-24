mod event_converter;

use crate::event_converter::convert_event;
use asn_core::traits::TAsnBaseEngine;
use asn_logger::trace;
use winit::event::Event;
use winit::event_loop::{EventLoop, EventLoopWindowTarget};
use winit::window::{Window, WindowBuilder};

struct RunnerDataset {
    // window: &'d Window,
}

pub fn run_loop<E>(e: &mut E)
where
    E: TAsnBaseEngine,
{
    trace!("Engine:run");
    let event_loop = EventLoop::new().unwrap();
    // let window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut r = RunnerDataset {
        // window: &window
    };

    event_loop
        .run(|evt, t| custom_event_handler(e, evt, t, &mut r))
        .unwrap();
}

fn custom_event_handler<E>(
    e: &mut E,
    evt: Event<()>,
    t: &EventLoopWindowTarget<()>,
    r: &mut RunnerDataset,
) where
    E: TAsnBaseEngine,
{
    if e.is_need_exit() {
        t.exit();
        return;
    }
    match convert_event(&evt) {
        None => {}
        Some(e) => {
            trace!("AsnEvent: {:?}", e)
        }
    }
    // e.set_need_exit();
    trace!("custom_event_handler: {:?} {:?}", evt, t);
}
