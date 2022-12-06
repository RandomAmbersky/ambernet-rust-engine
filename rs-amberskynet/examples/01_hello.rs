mod view_2d;

use crate::view_2d::View2D;
use rs_amberskynet::{AsnContext, ExtHandlerTrait};

struct Handler {
    view_2d: View2D,
}

impl Handler {
    pub fn new(ctx: &AsnContext) -> Self {
        let texture_format = ctx
            .gfx
            .main_window
            .surface
            .get_supported_formats(&ctx.gfx.adapter)[0];
        let view_2d = View2D::new(&ctx.gfx.device, &ctx.gfx.queue, texture_format);
        Self { view_2d }
    }
}

impl ExtHandlerTrait for Handler {
    fn draw(&self, e: &rs_amberskynet::AsnContext) {
        // todo!()
    }

    fn update(&self, e: &rs_amberskynet::AsnContext) {
        // todo!()
    }
}

pub fn main() {
    let (ctx, event_loop) = rs_amberskynet::init();
    let h = Handler::new(&ctx);
    rs_amberskynet::run(ctx, event_loop, h)
}
