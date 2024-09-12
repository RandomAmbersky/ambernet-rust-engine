mod application_handler;

use asn_core::traits::{TAsnBaseEngine, TAsnHandler};
use asn_core_winapi::TAsnRenderManager;
use asn_wgpu_released::WinitAdapter;
use std::sync::Arc;
use winit::window::Window;

pub struct RunnerDataset<'a, E, H, R>
where
    E: TAsnBaseEngine,
    H: TAsnHandler<E>,
    R: TAsnRenderManager + WinitAdapter,
{
    window: Option<Arc<Window>>,
    e: &'a mut E,
    h: &'a mut H,
    m: &'a mut R,
}

pub fn new_runner_dataset<'a, E, H, R>(
    e: &'a mut E,
    h: &'a mut H,
    m: &'a mut R,
) -> RunnerDataset<'a, E, H, R>
where
    E: TAsnBaseEngine,
    H: TAsnHandler<E>,
    R: TAsnRenderManager + WinitAdapter,
{
    RunnerDataset {
        window: None,
        e,
        h,
        m,
    }
}
