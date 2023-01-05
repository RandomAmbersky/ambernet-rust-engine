use rs_amberskynet::gfx::Vertex;
use wgpu::{BindGroupLayout, Device, RenderPipeline, ShaderModule, TextureFormat, TextureView};

pub fn get_color_attachment(view: &TextureView) -> wgpu::RenderPassColorAttachment {
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
    texture_bind_group_layout: &BindGroupLayout,
) -> RenderPipeline {
    let desc = wgpu::PipelineLayoutDescriptor {
        label: Some("Render Pipeline Layout"),
        bind_group_layouts: &[texture_bind_group_layout],
        push_constant_ranges: &[],
    };

    let render_pipeline_layout = device.create_pipeline_layout(&desc);

    let target_state = wgpu::ColorTargetState {
        format,
        blend: Some(wgpu::BlendState {
            color: wgpu::BlendComponent::REPLACE,
            alpha: wgpu::BlendComponent::REPLACE,
        }),
        write_mask: wgpu::ColorWrites::ALL,
    };

    let vertex_state = wgpu::VertexState {
        module: shader,
        entry_point: "vs_main",
        buffers: &[Vertex::desc()],
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
