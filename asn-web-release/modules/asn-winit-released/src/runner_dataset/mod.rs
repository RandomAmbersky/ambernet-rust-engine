mod application_handler;

use asn_core::traits::{TAsnBaseEngine, TAsnHandler};
use winit::window::Window;

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
