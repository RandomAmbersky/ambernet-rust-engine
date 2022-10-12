use std::iter;
use wgpu::{Device, Queue, SurfaceTexture};

pub struct ColorQuad {}

impl ColorQuad {
	pub fn new(device: &Device, queue: &Queue, config: &wgpu::SurfaceConfiguration) -> Self {
		Self {}
	}

	pub fn draw(&self, device: &Device, queue: &Queue, output: &SurfaceTexture) {
		let mut encoder = device
			.create_command_encoder(&wgpu::CommandEncoderDescriptor {
				label: Some("Render Encoder"),
			});

		let view = output
			.texture
			.create_view(&wgpu::TextureViewDescriptor::default());

		{
			let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
				label: Some("Render Pass"),
				color_attachments: &[Some(wgpu::RenderPassColorAttachment {
					view: &view,
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
		}

		queue.submit(iter::once(encoder.finish()));
	}

}
