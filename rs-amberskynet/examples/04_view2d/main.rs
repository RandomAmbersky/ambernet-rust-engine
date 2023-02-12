mod view_2d;

pub const TEXTURE_SOURCE: &[u8] = include_bytes!("./resource/tiles_mod.png");

use crate::view_2d::View2D;
use rs_amberskynet::core_gfx::texture::AsnTextureTrait;
use rs_amberskynet::gfx::gfx_error::GfxError;
use rs_amberskynet::gfx::AsnTexture;
use rs_amberskynet::{AsnContext, ExtHandlerTrait};

struct Handler {
    view_2d: View2D,
}

impl Handler {
    pub fn new(ctx: &AsnContext) -> Result<Self, GfxError> {
        let format = ctx.gfx.main_window.get_format(&ctx.gfx.adapter);

        let texture = AsnTexture::from_raw_image(&ctx.gfx, TEXTURE_SOURCE)?;

        let view_2d = View2D::new(&ctx.gfx, &texture, format)?;

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
    let (ctx, event_loop) = rs_amberskynet::init();
    if let Ok(t) = Handler::new(&ctx) {
        rs_amberskynet::run(ctx, event_loop, t)
    };
}
