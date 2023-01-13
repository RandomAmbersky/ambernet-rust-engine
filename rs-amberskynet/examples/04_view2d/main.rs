mod view_2d;

pub const TEXTURE_SOURCE: &[u8] = include_bytes!("./resource/tiles_mod.png");

use crate::view_2d::View2D;
use rs_amberskynet::gfx::AsnTexture;
use rs_amberskynet::{AsnContext, ExtHandlerTrait};

struct Handler {
    view_2d: View2D,
}

impl Handler {
    pub fn new(ctx: &AsnContext) -> Self {
        let format = ctx.gfx.main_window.get_format(&ctx.gfx.adapter);

        let texture = AsnTexture::from_bytes(
            &ctx.gfx.device,
            &ctx.gfx.queue,
            TEXTURE_SOURCE,
            "Tiles Mod Texture",
        )
        .unwrap();

        let view_2d = View2D::new(&ctx.gfx.device, &ctx.gfx.queue, &texture, format);

        Self { view_2d }
    }
}

impl ExtHandlerTrait for Handler {
    fn draw(&mut self, ctx: &mut AsnContext) {
        let fcx = ctx.gfx.fcx.as_mut().unwrap();
        self.view_2d
            .draw(&ctx.gfx.queue, &mut fcx.encoder, &fcx.view);
    }
    fn update(&mut self, _e: &mut AsnContext) {
        self.view_2d.update();
    }
}

pub fn main() {
    let (ctx, event_loop) = rs_amberskynet::init();
    let h = Handler::new(&ctx);
    rs_amberskynet::run(ctx, event_loop, h)
}
