mod view_2d;

use crate::view_2d::View2D;
use cgmath::num_traits::ToPrimitive;
// use asn_logger::AsnLogLevel;

use rs_amberskynet::{AsnContext, ExtHandlerTrait};
use rs_core::Size2D;
use rs_gfx_wgpu::gfx_error::GfxError;

struct Handler {
    view_2d: View2D,
}

// const GLOBAL_LOG_FILTER: AsnLogLevel = AsnLogLevel::Debug;

impl Handler {
    pub fn new(ctx: &AsnContext) -> Result<Self, GfxError> {
        let format = ctx.gfx.main_window.get_format(&ctx.gfx.adapter);
        let mut view_2d = View2D::new(&ctx.gfx, format)?;
        view_2d.update().expect("panic message");
        Ok(Self { view_2d })
    }
}

impl ExtHandlerTrait for Handler {
    fn draw(&mut self, ctx: &mut AsnContext) {
        self.view_2d.draw(&mut ctx.gfx);
    }
    fn update(&mut self, _e: &mut AsnContext) {
        self.view_2d.update().expect("update error");
    }
}

pub fn main() {
    // asn_logger::init_log(GLOBAL_LOG_FILTER);
    let (ctx, event_loop) = rs_amberskynet::init();
    if let Ok(_t) = Handler::new(&ctx) {
        rs_amberskynet::run(ctx, event_loop, _t)
    };
}
