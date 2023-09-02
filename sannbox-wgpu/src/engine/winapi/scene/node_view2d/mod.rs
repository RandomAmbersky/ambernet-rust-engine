mod resource;

use cgmath::{Matrix4, One, Vector3};
use crate::engine::winapi::defines;
use crate::engine::winapi::defines::{BytesArray, CellSize};
use crate::engine::winapi::mesh::Mesh;
use crate::engine::winapi::scene::node_view2d::resource::{
    Vertex, INDICES, SHADER_SOURCE, VERTICES,
};
use crate::engine::winapi::utils::ToWgpuFormat;
use crate::engine::winapi::wgpu::bind_groups::{BindGroupEntryBuilder, BindGroupLayoutBuilder};
use crate::engine::winapi::wgpu::texture::AsnTexture;
use crate::engine::winapi::wgpu::{AsnWgpuFrameContext, AsnWgpuWinApi};
use asn_core::errors::AsnRenderError;
use asn_core::math::{Pos2D, Size2D};
use asn_core::winapi::scene::{TNodeBase, TNodeView2d};
use asn_core::winapi::{AsnTextureFormat, TAsnWinapi, TTexture};
use wgpu::util::DeviceExt;
use wgpu::{BindGroupLayout, Device, ShaderModule, TextureFormat};

struct RenderState {
    render_pipeline: wgpu::RenderPipeline,
    bind_group: wgpu::BindGroup,
    map_setup_bind_group: wgpu::BindGroup,
    mesh: Mesh,
}

struct ViewState {
    view_texture: AsnTexture,
    view: BytesArray,
}

pub struct AsnWgpuNodeView2d {
    base_uniform: NodeBaseUniform,
    render_state: RenderState,
    view_state: ViewState,
    is_need_update: bool,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct MapSetupUniform {
    u_map_size: [f32; 2],
    u_tile_size: [f32; 2],
    u_sheet_size: [f32; 2],
    max_color_value: f32,
    _padding: u32, // stupid manual aligned, look at https://sotrh.github.io/learn-wgpu/news/0.12/#multi-view-added
}

struct NodeBaseUniform {
    pos: Vector3<f32>,
    rot: Vector3<f32>,
    scale: Vector3<f32>,
    prs: Matrix4<f32>
}

fn create_map_setup_bind_group(
    d: &wgpu::Device,
    tile_texture_size_in_pixels: &Size2D<u32>,
    map_size_in_tiles: &Size2D<u32>,
    tile_size_in_pixels: &Size2D<u32>,
) -> (wgpu::BindGroup, wgpu::BindGroupLayout) {
    let map_setup_uniform = MapSetupUniform {
        u_map_size: [
            map_size_in_tiles.width as f32,
            map_size_in_tiles.height as f32,
        ], // 8
        u_tile_size: [
            tile_size_in_pixels.width as f32,
            tile_size_in_pixels.height as f32,
        ],
        u_sheet_size: [
            tile_texture_size_in_pixels.width as f32,
            tile_texture_size_in_pixels.height as f32,
        ],
        max_color_value: 255.0 * 2.0 * 2.0 * 2.0 * 2.0,
        _padding: 0,
    };

    let var = bytemuck::bytes_of(&map_setup_uniform.max_color_value);
    println!("VAR LENGTH: {:?}", var.len());
    let var = bytemuck::bytes_of(&map_setup_uniform);
    println!("VAR LENGTH: {:?}", var.len());

    let map_setup_buffer = d.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("map_setup_buffer"),
        contents: bytemuck::bytes_of(&map_setup_uniform),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });

    let map_setup_bind_group_layout =
        d.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: Some("map_setup_bind_group_layout"),
        });

    let map_setup_bind_group = d.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: &map_setup_bind_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: map_setup_buffer.as_entire_binding(),
        }],
        label: Some("map_setup_bind_group"),
    });

    (map_setup_bind_group, map_setup_bind_group_layout)
}

fn create_diffuse_bind_group(
    d: &wgpu::Device,
    texture: &AsnTexture,
    tile_texture: &AsnTexture,
) -> (wgpu::BindGroup, wgpu::BindGroupLayout) {
    let group_layout_builder = BindGroupLayoutBuilder::new()
        .texture()
        .sampler()
        .texture()
        .sampler();
    let group_layout_desc = wgpu::BindGroupLayoutDescriptor {
        entries: group_layout_builder.entries(),
        label: Some("texture_bind_group_layout"),
    };
    let diffuse_bind_group_layout = d.create_bind_group_layout(&group_layout_desc);
    // let bind_group_layouts = &[&diffuse_bind_group_layout];

    let group_entry_builder = BindGroupEntryBuilder::default()
        .texture(&texture.view)
        .sampler(&texture.sampler)
        .texture(&tile_texture.view)
        .sampler(&tile_texture.sampler);

    let group_desc = wgpu::BindGroupDescriptor {
        layout: &diffuse_bind_group_layout,
        entries: group_entry_builder.entries(),
        label: Some("diffuse_bind_group"),
    };
    let diffuse_bind_group = d.create_bind_group(&group_desc);
    (diffuse_bind_group, diffuse_bind_group_layout)
}

fn create_shader(d: &wgpu::Device) -> wgpu::ShaderModule {
    let shader = d.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Shader"),
        source: wgpu::ShaderSource::Wgsl(SHADER_SOURCE.into()),
    });
    shader
}

fn draw_render_state(fcx: &mut AsnWgpuFrameContext, r: &RenderState) {
    let mut render_pass = fcx.encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: Some("Render Pass"),
        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
            view: &fcx.view,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(wgpu::Color {
                    r: 0.1,
                    g: 0.2,
                    b: 0.3,
                    a: 1.0,
                }),
                store: true,
            },
        })],
        depth_stencil_attachment: None,
    });
    render_pass.set_pipeline(&r.render_pipeline);
    render_pass.set_bind_group(0, &r.bind_group, &[]);
    render_pass.set_bind_group(1, &r.map_setup_bind_group, &[]);
    render_pass.set_vertex_buffer(0, r.mesh.vertex_buffer.slice(..));
    render_pass.set_index_buffer(r.mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
    render_pass.draw_indexed(0..r.mesh.num_indices, 0, 0..1);
}

fn set_cell(
    v: &mut ViewState,
    pos: &Pos2D<<AsnWgpuNodeView2d as TNodeView2d>::SizeDimension>,
    c: CellSize,
) {
    if !v.view.size.is_pos_into(pos) {
        panic!("Pos {:?} not in view size {:?}", pos, v.view.size)
    }

    let cell_y: u8 = c / 16;
    let cell_x: u8 = c - cell_y * 16;

    let index = ((pos.y * v.view.size.width + pos.x) * 4) as usize;

    v.view.bytes[index] = cell_x;
    v.view.bytes[index + 1] = cell_y;
    // println!("set cell {:?} {:?} {:?} {:?}", pos, cell_y, cell_x, c);
}

impl TNodeBase for AsnWgpuNodeView2d {
    type FrameContext = AsnWgpuFrameContext;
    type WinApi = AsnWgpuWinApi;
    type AsnTexture = AsnTexture;
    fn draw(&mut self, fcx: &mut Self::FrameContext) {
        draw_render_state(fcx, &self.render_state);
    }

    fn update(&mut self, gfx: &mut Self::WinApi) {
        if !self.is_need_update {
            return;
        }

        self.view_state
            .view_texture
            .update_from_raw(gfx, &self.view_state.view.bytes)
            .unwrap();

        self.is_need_update = false;
    }
}

impl TNodeView2d for AsnWgpuNodeView2d {
    type CellType = defines::CellSize;
    type SizeDimension = defines::SizeDimension;

    fn new(
        gfx: &mut Self::WinApi,
        tile_texture: &Self::AsnTexture,
        view_size_in_tiles: &Size2D<u32>,
        tile_size_in_pixels: &Size2D<u32>,
    ) -> Self {
        let mesh = Mesh::build(bytemuck::cast_slice(VERTICES), INDICES, gfx.get_device());

        let shader = create_shader(gfx.get_device());

        let texture_format = gfx.get_config().texture_format.to_wgpu_format();
        println!("texure format: {:?}", texture_format);

        let view = BytesArray {
            size: Size2D {
                width: view_size_in_tiles.width,
                height: view_size_in_tiles.height,
            },
            bytes: vec![0_u8; (view_size_in_tiles.get_size() * 4) as usize],
        };
        let view_texture =
            AsnTexture::from_raw(gfx, &view.bytes, &view.size, AsnTextureFormat::Rgba8).unwrap();

        let (diffuse_bind_group, diffuse_bind_group_layout) =
            create_diffuse_bind_group(gfx.get_device(), &view_texture, &tile_texture);

        let (map_setup_bind_group, map_setup_bind_group_layout) = create_map_setup_bind_group(
            gfx.get_device(),
            &tile_texture.get_size(),
            &view_size_in_tiles,
            &tile_size_in_pixels,
        );

        let bind_group_layouts = &[&diffuse_bind_group_layout, &map_setup_bind_group_layout];

        let render_pipeline = get_render_pipeline(
            gfx.get_device(),
            texture_format,
            &shader,
            &bind_group_layouts,
        );

        // let (render_pipeline, bind_group) =
        //     create_node_view2d_set(gfx, &view_texture, &tile_texture, texture_format, &shader);

        let render_state = RenderState {
            render_pipeline,
            bind_group: diffuse_bind_group,
            map_setup_bind_group,
            mesh,
        };
        let view_state = ViewState { view, view_texture };

        let base_uniform = NodeBaseUniform {
            pos: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
            rot: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
            scale: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
            prs: Matrix4::one()
        };

        println!("{:?}", base_uniform.prs);

        Self {
            base_uniform,
            render_state,
            view_state,
            is_need_update: false,
        }
    }

    fn set_cell(
        &mut self,
        pos: &Pos2D<Self::SizeDimension>,
        c: CellSize,
    ) -> Result<(), AsnRenderError> {
        // if !self.view_state.view.size.is_pos_into(pos) {
        //     panic!(
        //         "Pos {:?} not in view size {:?}",
        //         pos, self.view_state.view.size
        //     )
        // }
        //
        // let cell_y: u8 = c / 16;
        // let cell_x: u8 = c - cell_y * 16;
        //
        // let index = ((pos.y * self.view_state.view.size.width + pos.x) * 4) as usize;
        //
        // self.view_state.view.bytes[index] = cell_x;
        // self.view_state.view.bytes[index + 1] = cell_y;

        set_cell(&mut self.view_state, pos, c);
        self.is_need_update = true;
        Ok(())
    }

    fn update_from_raw(&mut self, bytes: &[u8]) -> Result<(), AsnRenderError> {
        update_view_from_raw(&mut self.view_state, bytes);
        self.is_need_update = true;
        Ok(())
    }
}

fn update_view_from_raw(v: &mut ViewState, buf: &[u8]) {
    let mut index = buf.iter();
    for y in 0..v.view.size.height {
        for x in 0..v.view.size.width {
            let c = *index.next().unwrap() - 1;
            set_cell(v, &Pos2D { x, y }, c);
        }
    }
}

fn get_render_pipeline(
    device: &Device,
    format: TextureFormat,
    shader: &ShaderModule,
    bind_group_layouts: &[&BindGroupLayout; 2],
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
        buffers: &[Vertex::desc()],
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
