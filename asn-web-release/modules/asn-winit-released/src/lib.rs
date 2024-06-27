use asn_core::events::{AsnEvent, AsnEventEmitter};
use asn_core::traits::{TAsnBaseEngine, TAsnHandler};
use asn_logger::trace;
use winit::event::Event;
use winit::event_loop::{EventLoop, EventLoopProxy};
use winit::window::Window;

mod application_handler;
mod event_converter;

pub struct RunnerDataset<'a, E, H>
where
    E: TAsnBaseEngine,
    H: TAsnHandler<E>,
{
    window: Option<Window>,
    event_loop_proxy: Option<EventLoopProxy<Event<()>>>,
    e: &'a mut E,
    h: &'a mut H,
}

pub fn new_runner_dataset<'a, E, H>(e: &'a mut E, h: &'a mut H) -> RunnerDataset<'a, E, H>
where
    E: TAsnBaseEngine,
    H: TAsnHandler<E>,
{
    RunnerDataset {
        event_loop_proxy: None,
        window: None,
        e,
        h,
    }
}

pub fn run_loop<E, H>(e: &mut E, h: &mut H)
where
    E: TAsnBaseEngine + AsnEventEmitter,
    H: TAsnHandler<E>,
{
    trace!("run_loop:run");
    let event_loop = EventLoop::<Event<()>>::with_user_event().build().unwrap();

    let event_loop_proxy: EventLoopProxy<Event<()>> = event_loop.create_proxy();

    let mut r = new_runner_dataset(e, h);
    r.event_loop_proxy = Some(event_loop_proxy);
    event_loop.run_app(&mut r).unwrap()
}
