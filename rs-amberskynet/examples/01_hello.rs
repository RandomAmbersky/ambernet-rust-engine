mod view_2d;

use crate::view_2d::View2D;
use rs_amberskynet::{AsnContext, ExtHandlerTrait};
use std::iter;

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
    fn draw(&self, ctx: &AsnContext) {
        let frame = ctx.gfx.main_window.get_current_texture();

        let mut encoder = ctx
            .gfx
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder View2D"),
            });

        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        self.view_2d.draw(&mut encoder, &view);

        ctx.gfx.queue.submit(iter::once(encoder.finish()));

        frame.present();
    }

    fn update(&self, e: &AsnContext) {
        // todo!()
    }
}

pub fn main() {
    let (ctx, event_loop) = rs_amberskynet::init();
    let h = Handler::new(&ctx);
    rs_amberskynet::run(ctx, event_loop, h)
}
