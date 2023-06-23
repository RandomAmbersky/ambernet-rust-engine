mod view_2d;

use std::time::{Duration, Instant};

pub const TEXTURE_SOURCE: &[u8] = include_bytes!("./resource/tiles_mod.png");

use crate::view_2d::View2D;
// use asn_logger::AsnLogLevel;

use rs_amberskynet::{AsnContext, ExtHandlerTrait};
use rs_gfx_core::{AsnTextureFormat, AsnTextureTrait};
use rs_gfx_wgpu::gfx_error::GfxError;
use rs_gfx_wgpu::AsnTexture;

struct Handler {
    view_2d: View2D,
    delta_time: Instant,
}

const DURATION: Duration = Duration::from_millis(10); // Сколько вам нужно секунд.

// const GLOBAL_LOG_FILTER: AsnLogLevel = AsnLogLevel::Debug;

impl Handler {
    pub fn new(ctx: &AsnContext) -> Result<Self, GfxError> {
        let format = ctx.gfx.main_window.get_format(&ctx.gfx.adapter);
        let texture =
            AsnTexture::from_raw_image(&ctx.gfx, TEXTURE_SOURCE, AsnTextureFormat::Rgba8)?;
        let mut view_2d = View2D::new(&ctx.gfx, texture, format)?;
        view_2d.update().expect("panic message");
        Ok(Self {
            view_2d,
            delta_time: Instant::now(),
        })
    }
}

impl ExtHandlerTrait for Handler {
    fn draw(&mut self, ctx: &mut AsnContext) {
        self.view_2d.draw(&mut ctx.gfx);
    }
    fn update(&mut self, _e: &mut AsnContext) {
        let now = Instant::now();
        if now - self.delta_time >= DURATION {
            self.delta_time = now;
            self.view_2d.update().expect("update error");
        }
    }
}

pub fn main() {
    // asn_logger::init_log(GLOBAL_LOG_FILTER);

    let (ctx, event_loop) = rs_amberskynet::init();
    if let Ok(_t) = Handler::new(&ctx) {
        rs_amberskynet::run(ctx, event_loop, _t)
    };
}
