mod mesh;
mod model_vertex;

use crate::view_2d::mesh::Mesh;
use model_vertex::ModelVertex;
use model_vertex::{INDICES, VERTICES};
use rand::Rng;
use rs_gfx_wgpu::{AsnGfx, AsnTexture, BindGroupEntryBuilder, BindGroupLayoutBuilder};
use wgpu::{BindGroupLayout, Device, ShaderModule, TextureFormat, TextureView};

use rs_core::{Array2D, Pos2D};
use rs_gfx_core::AsnTextureTrait;
use rs_gfx_wgpu::defines::{BytesArray, Size2d, SizeDimension};
use rs_gfx_wgpu::gfx_error::GfxError;

pub const SHADER_SOURCE: &str = include_str!("shader.wgsl");
// const ONE_BLUE_PIXEL: [u8; 4] = [0, 0, 255, 255];
// const TWO_PIXEL: [u8; 8] = [0, 0, 255, 255, 255, 0, 0, 255];
// const FOUR_PIXEL: [u8; 16] = [
//     0, 0, 255, 255, 255, 0, 0, 255, 255, 0, 255, 255, 0, 255, 255, 255,
// ];

pub struct View2D {
    texture: AsnTexture,
    view: BytesArray,
    mesh: Mesh,
    bind_group: wgpu::BindGroup,
    render_pipeline: wgpu::RenderPipeline,
    is_need_update: bool,
}

impl View2D {
    pub fn new(
        gfx: &AsnGfx,
        _texture: &AsnTexture,
        format: TextureFormat,
    ) -> Result<Self, GfxError> {
        let mesh = Mesh::build(VERTICES, INDICES, &gfx.device);

        let shader = gfx
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("Shader"),
                source: wgpu::ShaderSource::Wgsl(SHADER_SOURCE.into()),
            });

        let group_layout_builder = BindGroupLayoutBuilder::new().texture().sampler();
        let group_layout_desc = wgpu::BindGroupLayoutDescriptor {
            entries: group_layout_builder.entries(),
            label: Some("texture_bind_group_layout"),
        };
        let diffuse_bind_group_layout = gfx.device.create_bind_group_layout(&group_layout_desc);
        let bind_group_layouts = &[&diffuse_bind_group_layout];

        let render_pipeline = get_render_pipeline(&gfx.device, format, &shader, bind_group_layouts);

        let texture_size_w: u32 = 3200 / 16;
        let texture_size_h: u32 = 2400 / 16;

        let view = Array2D {
            size: Size2d {
                width: texture_size_w as SizeDimension,
                height: texture_size_h as SizeDimension,
            },
            bytes: vec![0; (texture_size_w * texture_size_h * 4) as usize],
        };

        let texture = AsnTexture::from_array(gfx, &view)?;

        let group_entry_builder = BindGroupEntryBuilder::default()
            .texture(&texture.view)
            .sampler(&texture.sampler);
        let group_desc = wgpu::BindGroupDescriptor {
            layout: &diffuse_bind_group_layout,
            entries: group_entry_builder.entries(),
            label: Some("diffuse_bind_group"),
        };
        let bind_group = gfx.device.create_bind_group(&group_desc);

        let view_2d = Self {
            texture,
            view,
            mesh,
            bind_group,
            render_pipeline,
            is_need_update: false,
        };
        Ok(view_2d)
    }
    pub fn draw(&mut self, gfx: &mut AsnGfx) {
        if self.is_need_update {
            self.is_need_update = false;
            self.texture
                .update_from_array(gfx, &self.view)
                .expect("TODO: panic message");
        }
        let fcx = gfx.fcx.as_mut().unwrap();
        let mut render_pass = fcx.encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(get_color_attachment(&fcx.view))],
            depth_stencil_attachment: None,
        });
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &self.bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.mesh.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..self.mesh.num_indices, 0, 0..1);
    }
    pub fn update(&mut self) -> Result<(), String> {
        let mut rng = rand::thread_rng();

        for _ in 0..1000 {
            let pos = Pos2D {
                x: rng.gen_range(0..self.view.size.width),
                y: rng.gen_range(0..self.view.size.height),
            };

            let index = self.view.get_point(&pos)?;
            //     // let index = pos_y * self.view.size.width + pos_x;
            //
            let byte_index = index * 4 + rng.gen_range(0..4);
            //
            // let value: u8 = rng.gen();
            let mut value: u8 = self.view.bytes[byte_index as usize];

            if rng.gen_range(0..100) > 50 {
                value = value.wrapping_sub(1);
            } else {
                value = value.wrapping_add(rng.gen_range(1..50));
            }
            self.view.bytes[byte_index as usize] = value;
        }

        self.is_need_update = true;
        Ok(())
    }
}

fn get_color_attachment(view: &TextureView) -> wgpu::RenderPassColorAttachment {
    let clear_color = wgpu::Color {
        r: 0.1,
        g: 0.2,
        b: 0.3,
        a: 1.0,
    };
    wgpu::RenderPassColorAttachment {
        view,
        resolve_target: None,
        ops: wgpu::Operations {
            load: wgpu::LoadOp::Clear(clear_color),
            store: true,
        },
    }
}

pub fn get_render_pipeline(
    device: &Device,
    format: TextureFormat,
    shader: &ShaderModule,
    bind_group_layouts: &[&BindGroupLayout],
) -> wgpu::RenderPipeline {
    let desc = wgpu::PipelineLayoutDescriptor {
        label: Some("Render Pipeline Layout"),
        bind_group_layouts,
        push_constant_ranges: &[],
    };

    let render_pipeline_layout = device.create_pipeline_layout(&desc);

    let vertex_state = wgpu::VertexState {
        module: shader,
        entry_point: "vs_main",
        buffers: &[ModelVertex::desc()],
    };

    let target_state = wgpu::ColorTargetState {
        format,
        blend: Some(wgpu::BlendState {
            color: wgpu::BlendComponent::REPLACE,
            alpha: wgpu::BlendComponent::REPLACE,
        }),
        write_mask: wgpu::ColorWrites::ALL,
    };

    let fragment_state = wgpu::FragmentState {
        module: shader,
        entry_point: "fs_main",
        targets: &[Some(target_state)],
    };

    let primitive = wgpu::PrimitiveState {
        topology: wgpu::PrimitiveTopology::TriangleList,
        strip_index_format: None,
        front_face: wgpu::FrontFace::Ccw,
        cull_mode: Some(wgpu::Face::Back),
        // Setting this to anything other than Fill requires Features::POLYGON_MODE_LINE
        // or Features::POLYGON_MODE_POINT
        polygon_mode: wgpu::PolygonMode::Fill,
        // Requires Features::DEPTH_CLIP_CONTROL
        unclipped_depth: false,
        // Requires Features::CONSERVATIVE_RASTERIZATION
        conservative: false,
    };

    let multisample = wgpu::MultisampleState {
        count: 1,
        mask: !0,
        alpha_to_coverage_enabled: false,
    };

    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Render Pipeline"),
        layout: Some(&render_pipeline_layout),
        vertex: vertex_state,
        fragment: Some(fragment_state),
        primitive,
        depth_stencil: None,
        multisample,
        // If the pipeline will be used with a multiview render pass, this
        // indicates how many array layers the attachments will have.
        multiview: None,
    })
}
