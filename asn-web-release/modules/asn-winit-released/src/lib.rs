mod application_handler;
mod event_converter;

use asn_core::traits::{TAsnBaseEngine, TAsnHandler};
use asn_logger::trace;
use winit::event::{DeviceEvent, DeviceId, Event};
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Window, WindowId};

struct RunnerDataset<'a, E, H>
where
    E: TAsnBaseEngine,
    H: TAsnHandler<E>,
{
    window: Option<Window>,
    counter: i32,
    e: &'a mut E,
    h: &'a mut H,
}

pub fn run_loop<E, H>(e: &mut E, h: &mut H)
where
    E: TAsnBaseEngine,
    H: TAsnHandler<E>,
{
    trace!("Engine:run");
    let event_loop = EventLoop::new().unwrap();
    let mut state = RunnerDataset {
        window: None,
        counter: 0,
        e,
        h,
    };
    let _ = event_loop.run_app(&mut state);
}
