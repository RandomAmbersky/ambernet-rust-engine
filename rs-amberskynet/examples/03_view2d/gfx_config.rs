use rs_amberskynet::gfx::{AsnTexture, Vertex};
use wgpu::{BindGroupLayout, Device, RenderPipeline, ShaderModule, TextureFormat, TextureView};

pub fn get_multisample_state() -> wgpu::MultisampleState {
    wgpu::MultisampleState {
        count: 1,
        mask: !0,
        alpha_to_coverage_enabled: false,
    }
}

pub fn get_primitive_state() -> wgpu::PrimitiveState {
    wgpu::PrimitiveState {
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
    }
}

pub fn get_color_target_state(format: TextureFormat) -> wgpu::ColorTargetState {
    wgpu::ColorTargetState {
        format,
        blend: Some(wgpu::BlendState {
            color: wgpu::BlendComponent::REPLACE,
            alpha: wgpu::BlendComponent::REPLACE,
        }),
        write_mask: wgpu::ColorWrites::ALL,
    }
}

pub fn get_clear_color() -> wgpu::Color {
    wgpu::Color {
        r: 0.1,
        g: 0.2,
        b: 0.3,
        a: 1.0,
    }
}

pub fn get_color_attachment(view: &TextureView) -> wgpu::RenderPassColorAttachment {
    wgpu::RenderPassColorAttachment {
        view,
        resolve_target: None,
        ops: wgpu::Operations {
            load: wgpu::LoadOp::Clear(get_clear_color()),
            store: true,
        },
    }
}

pub fn get_render_pipeline(
    device: &Device,
    format: TextureFormat,
    shader: &ShaderModule,
    texture_bind_group_layout: &BindGroupLayout,
) -> RenderPipeline {
    let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Render Pipeline Layout"),
        bind_group_layouts: &[texture_bind_group_layout],
        push_constant_ranges: &[],
    });

    let vertex_state = wgpu::VertexState {
        module: shader,
        entry_point: "vs_main",
        buffers: &[Vertex::desc()],
    };

    let fragment_state = wgpu::FragmentState {
        module: shader,
        entry_point: "fs_main",
        targets: &[Some(get_color_target_state(format))],
    };

    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Render Pipeline"),
        layout: Some(&render_pipeline_layout),
        vertex: vertex_state,
        fragment: Some(fragment_state),
        primitive: get_primitive_state(),
        depth_stencil: None,
        multisample: get_multisample_state(),
        // If the pipeline will be used with a multiview render pass, this
        // indicates how many array layers the attachments will have.
        multiview: None,
    })
}

pub fn get_bind_group_layout_descriptor<'a>() -> wgpu::BindGroupLayoutDescriptor<'a> {
    wgpu::BindGroupLayoutDescriptor {
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    multisampled: false,
                    view_dimension: wgpu::TextureViewDimension::D2,
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            },
        ],
        label: Some("texture_bind_group_layout"),
    }
}

// pub fn get_group_descriptor<'a>(
//     texture: &'static AsnTexture,
//     layout: &'static BindGroupLayout,
// ) -> wgpu::BindGroupDescriptor<'a> {
//     let enteries = [
//         wgpu::BindGroupEntry {
//             binding: 0,
//             resource: wgpu::BindingResource::TextureView(&texture.view),
//         },
//         wgpu::BindGroupEntry {
//             binding: 1,
//             resource: wgpu::BindingResource::Sampler(&texture.sampler),
//         },
//     ];
//     wgpu::BindGroupDescriptor {
//         layout,
//         entries: &enteries,
//         label: Some("diffuse_bind_group"),
//     }
// }
