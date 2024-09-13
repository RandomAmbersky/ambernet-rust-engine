use asn_core::events::AsnEventEmitter;
use asn_core::traits::TAsnBaseEngine;
use asn_core_winapi::TAsnRenderManager;
use asn_wgpu_released::WinitAdapter;

pub trait Engine {
    fn run(&mut self);
}

struct MyEngine<E, R>
where
    E: TAsnBaseEngine + AsnEventEmitter,
    R: TAsnRenderManager + WinitAdapter,
{
    e: E,
    r: R,
}

impl<E, R> Engine for MyEngine<E, R>
where
    E: TAsnBaseEngine + AsnEventEmitter,
    R: TAsnRenderManager + WinitAdapter,
{
    fn run(&mut self) {
        todo!()
    }
}

pub fn init() -> impl Engine {
    let e = asn_engine_released::get_engine();
    let r = asn_wgpu_released::get_render_manager();
    MyEngine { e, r }
}
