use crate::runner_dataset::{new_runner_dataset, RunnerDataset};
use asn_core::events::AsnEventEmitter;
use asn_core::traits::{TAsnBaseEngine, TAsnHandler};
use asn_logger::trace;
use asn_wgpu_released::get_render_manager;
use winit::event_loop::EventLoop;

mod event_converter;
mod runner_dataset;

pub fn run_loop<E, H>(e: &mut E, h: &mut H)
where
    E: TAsnBaseEngine + AsnEventEmitter,
    H: TAsnHandler<E>,
{
    trace!("run_loop:run");
    let event_loop = EventLoop::new().unwrap();

    let mut m = get_render_manager();

    let mut r = new_runner_dataset(e, h, &mut m);
    event_loop.run_app(&mut r).unwrap()
}
