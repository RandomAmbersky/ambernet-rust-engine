use crate::handler::Handler;
use asn_core::events::AsnEventEmitter;
use asn_core::traits::TAsnBaseEngine;
use asn_core_winapi::{TAsnRenderManager, TAsnWindowManager};
use asn_winit_released::WinitWindow;
use std::sync::Arc;

pub trait Engine {
    fn run(&mut self);
}

struct MyEngine<E, R>
where
    E: TAsnBaseEngine + AsnEventEmitter,
    R: TAsnRenderManager + TAsnWindowManager<Window = WinitWindow>,
{
    e: Arc<E>,
    r: Arc<R>,
}

impl<E, R> Engine for MyEngine<E, R>
where
    E: TAsnBaseEngine + AsnEventEmitter,
    R: TAsnRenderManager + TAsnWindowManager<Window = WinitWindow>,
{
    fn run(&mut self) {
        let mut h = Handler {};
        // let mut e = Arc::clone(&self.e);
        // let mut r = Arc::clone(&self.r).as_ref();
        asn_winit_released::run_loop(&mut self.e, &mut h, &r);
    }
}

pub fn init() -> impl Engine {
    let e = asn_engine_released::get_engine();
    let r = asn_wgpu_released::get_render_manager();
    MyEngine {
        e: Arc::new(e),
        r: Arc::new(r),
    }
}
