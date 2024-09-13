use crate::handler::Handler;
use asn_core::events::AsnEventEmitter;
use asn_core::traits::TAsnBaseEngine;
use asn_core_winapi::TAsnRenderManager;
use asn_wgpu_released::WinitAdapter;

pub fn init() -> (
    impl TAsnBaseEngine + AsnEventEmitter,
    impl TAsnRenderManager + WinitAdapter,
) {
    let e = asn_engine_released::get_engine();
    let r = asn_wgpu_released::get_render_manager();
    (e, r)
}

pub fn run<E, R>(e: &mut E, r: &mut R)
where
    E: TAsnBaseEngine + AsnEventEmitter,
    R: TAsnRenderManager + WinitAdapter,
{
    // let mut e = asn_engine_released::get_engine();
    // let mut r = asn_wgpu_released::get_render_manager();
    let mut h = Handler {};
    asn_winit_released::run_loop(e, &mut h, r);
}
