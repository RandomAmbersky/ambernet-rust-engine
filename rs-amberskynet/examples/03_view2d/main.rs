extern crate core;

mod gfx_config;
mod resource;

use crate::gfx_config::{get_color_attachment, get_render_pipeline};
use crate::resource::{INDICES, SHADER_SOURCE, TEXTURE_SOURCE, VERTICES};
use rs_amberskynet::gfx::{AsnTexture, BindGroupEntryBuilder, BindGroupLayoutBuilder, Vertex};
use rs_amberskynet::{AsnContext, ExtHandlerTrait};
use wgpu::util::DeviceExt;

struct Handler {
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
    diffuse_bind_group: wgpu::BindGroup,
    render_pipeline: wgpu::RenderPipeline,
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

        let entries = BindGroupLayoutBuilder::new().texture().sampler();
        let desc = wgpu::BindGroupLayoutDescriptor {
            entries: entries.entries(),
            label: Some("texture_bind_group_layout"),
        };

        let texture_bind_group_layout = ctx.gfx.device.create_bind_group_layout(&desc);

        let entries = BindGroupEntryBuilder::new()
            .texture(&texture.view)
            .sampler(&texture.sampler);

        let diffuse_bind_group = ctx
            .gfx
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &texture_bind_group_layout,
                entries: entries.entries(),
                label: Some("diffuse_bind_group"),
            });

        let vertex_buffer = ctx
            .gfx
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(VERTICES),
                usage: wgpu::BufferUsages::VERTEX,
            });

        let index_buffer = ctx
            .gfx
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(INDICES),
                usage: wgpu::BufferUsages::INDEX,
            });
        let num_indices = INDICES.len() as u32;

        let render_pipeline =
            get_render_pipeline(&ctx.gfx.device, format, &shader, &texture_bind_group_layout);

        Self {
            render_pipeline,
            vertex_buffer,
            index_buffer,
            num_indices,
            diffuse_bind_group,
        }
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
            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.diffuse_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
        }
    }
    fn update(&mut self, _e: &mut AsnContext) {}
}

pub fn main() {
    let (ctx, event_loop) = rs_amberskynet::init();
    let h = Handler::new(&ctx);
    rs_amberskynet::run(ctx, event_loop, h)
}
