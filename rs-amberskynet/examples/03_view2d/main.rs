extern crate core;

mod gfx_config;
mod resource;
mod view_2d;

use crate::gfx_config::{get_color_attachment, get_render_pipeline};
use crate::resource::{INDICES, SHADER_SOURCE, TEXTURE_SOURCE, VERTICES};
use crate::view_2d::View2D;
use rs_amberskynet::gfx::{AsnTexture, BindGroupEntryBuilder, BindGroupLayoutBuilder, Vertex};
use rs_amberskynet::{AsnContext, ExtHandlerTrait};
use wgpu::util::DeviceExt;

struct Handler {
    view_2d: View2D,
}

impl Handler {
    pub fn new(ctx: &AsnContext) -> Self {
        let format = ctx
            .gfx
            .main_window
            .surface
            .get_supported_formats(&ctx.gfx.adapter)[0];

        let shader = ctx
            .gfx
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("Shader"),
                source: wgpu::ShaderSource::Wgsl(SHADER_SOURCE.into()),
            });

        let texture = AsnTexture::from_bytes(
            &ctx.gfx.device,
            &ctx.gfx.queue,
            TEXTURE_SOURCE,
            "Tiles Mod Texture",
        )
        .unwrap();

        let view_2d = View2D::new(&ctx.gfx.device, &texture, format, &shader);

        Self { view_2d }
    }
}

impl ExtHandlerTrait for Handler {
    fn draw(&mut self, ctx: &mut AsnContext) {
        let fcx = ctx.gfx.fcx.as_mut().unwrap();
        {
            let mut render_pass = fcx.encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(get_color_attachment(&fcx.view))],
                depth_stencil_attachment: None,
            });
            render_pass.set_pipeline(&self.view_2d.render_pipeline);
            render_pass.set_bind_group(0, &self.view_2d.diffuse_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.view_2d.vertex_buffer.slice(..));
            render_pass.set_index_buffer(
                self.view_2d.index_buffer.slice(..),
                wgpu::IndexFormat::Uint16,
            );
            render_pass.draw_indexed(0..self.view_2d.num_indices, 0, 0..1);
        }
    }
    fn update(&mut self, _e: &mut AsnContext) {}
}

pub fn main() {
    let (ctx, event_loop) = rs_amberskynet::init();
    let h = Handler::new(&ctx);
    rs_amberskynet::run(ctx, event_loop, h)
}
