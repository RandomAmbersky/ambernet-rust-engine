use crate::runner_dataset::new_runner_dataset;
use asn_core::events::AsnEventEmitter;
use asn_core::traits::{TAsnBaseEngine, TAsnHandler};
use asn_core_winapi::TAsnRenderManager;
use asn_logger::trace;
use asn_wgpu_released::WinitAdapter;
use winit::event_loop::EventLoop;

mod event_converter;
mod runner_dataset;

pub fn run_loop<E, H, R>(e: &mut E, h: &mut H, r: &mut R)
where
    E: TAsnBaseEngine + AsnEventEmitter,
    H: TAsnHandler<E>,
    R: TAsnRenderManager + WinitAdapter,
{
    trace!("run_loop:run");
    let event_loop = EventLoop::new().unwrap();

    let mut runner = new_runner_dataset(e, h, r);
    event_loop.run_app(&mut runner).unwrap()
}
