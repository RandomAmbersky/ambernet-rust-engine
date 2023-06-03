use rs_amberskynet::{AsnContext, ExtHandlerTrait};
use rs_gfx_wgpu::gfx_error::GfxError;

struct Handler {}

impl Handler {
    fn new(e: &AsnContext) -> Result<Self, GfxError> {
        let h = Handler {};
        Ok(h)
    }
}

impl ExtHandlerTrait for Handler {
    fn draw(&mut self, e: &mut AsnContext) {
        // todo!()
    }

    fn update(&mut self, e: &mut AsnContext) {
        // todo!()
    }
}

pub fn main() {
    let (ctx, event_loop) = rs_amberskynet::init();
    if let Ok(t) = Handler::new(&ctx) {
        rs_amberskynet::run(ctx, event_loop, t)
    };
}
