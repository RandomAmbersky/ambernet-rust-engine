use asn_core::traits::{TAsnBaseEngine, TAsnHandler};
use asn_logger::trace;
use winit::event_loop::EventLoop;
use winit::window::Window;

mod application_handler;
mod event_converter;

pub struct RunnerDataset<'a, E, H>
where
    E: TAsnBaseEngine,
    H: TAsnHandler<E>,
{
    window: Option<Window>,
    e: &'a mut E,
    h: &'a mut H,
}

pub fn new_runner_dataset<'a, E, H>(e: &'a mut E, h: &'a mut H) -> RunnerDataset<'a, E, H>
where
    E: TAsnBaseEngine,
    H: TAsnHandler<E>,
{
    RunnerDataset { window: None, e, h }
}

pub fn run_loop<E, H>(r: &mut RunnerDataset<E, H>)
where
    E: TAsnBaseEngine,
    H: TAsnHandler<E>,
{
    trace!("Engine:run");
    let event_loop = EventLoop::new().unwrap();
    event_loop.run_app(r).unwrap()
}
