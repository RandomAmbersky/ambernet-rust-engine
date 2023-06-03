extern crate core;

mod resource;
mod view_2d;

use crate::resource::{SHADER_SOURCE, TEXTURE_SOURCE};
use crate::view_2d::View2D;
use rs_amberskynet::{AsnContext, ExtHandlerTrait};
use rs_gfx_core::AsnTextureTrait;
use rs_gfx_wgpu::AsnTexture;

struct Handler {
    view_2d: View2D,
}

impl Handler {
    pub fn new(ctx: &AsnContext) -> Self {
        let surface_caps = ctx
            .gfx
            .main_window
            .surface
            .get_capabilities(&ctx.gfx.adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let shader = ctx
            .gfx
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("Shader"),
                source: wgpu::ShaderSource::Wgsl(SHADER_SOURCE.into()),
            });

        let texture = AsnTexture::from_raw_image(&ctx.gfx, TEXTURE_SOURCE).unwrap();

        let view_2d = View2D::new(&ctx.gfx.device, &texture, surface_format, &shader);

        Self { view_2d }
    }
}

impl ExtHandlerTrait for Handler {
    fn draw(&mut self, ctx: &mut AsnContext) {
        let fcx = ctx.gfx.fcx.as_mut().unwrap();
        self.view_2d.draw(&mut fcx.encoder, &fcx.view);
    }
    fn update(&mut self, _e: &mut AsnContext) {}
}

pub fn main() {
    let (ctx, event_loop) = rs_amberskynet::init();
    let h = Handler::new(&ctx);
    rs_amberskynet::run(ctx, event_loop, h)
}
