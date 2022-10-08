use std::iter;
use wgpu::CompositeAlphaMode;

use wgpu::util::DeviceExt;
use winit::event::WindowEvent;
use winit::window::Window;
use crate::{INDICES, texture, Vertex, VERTICES};
use crate::resource::{SHADER_SOURCE, TEXTURE_SOURCE};
use crate::view_2d::View2D;

pub struct State {
	surface: wgpu::Surface,
	device: wgpu::Device,
	queue: wgpu::Queue,
	config: wgpu::SurfaceConfiguration,
	pub(crate) size: winit::dpi::PhysicalSize<u32>,
	view_2d: View2D
}

impl State {
	pub(crate) async fn new(window: &Window) -> Self {
		let size = window.inner_size();

		// The instance is a handle to our GPU
		// BackendBit::PRIMARY => Vulkan + Metal + DX12 + Browser WebGPU
		let instance = wgpu::Instance::new(wgpu::Backends::all());
		let surface = unsafe { instance.create_surface(window) };
		let adapter = instance
			.request_adapter(&wgpu::RequestAdapterOptions {
				power_preference: wgpu::PowerPreference::default(),
				compatible_surface: Some(&surface),
				force_fallback_adapter: false,
			})
			.await
			.unwrap();
		let (device, queue) = adapter
			.request_device(
				&wgpu::DeviceDescriptor {
					label: None,
					features: wgpu::Features::empty(),
					// WebGL doesn't support all of wgpu's features, so if
					// we're building for the web we'll have to disable some.
					limits: if cfg!(target_arch = "wasm32") {
						wgpu::Limits::downlevel_webgl2_defaults()
					} else {
						wgpu::Limits::default()
					},
				},
				None, // Trace path
			)
			.await
			.unwrap();

		let config = wgpu::SurfaceConfiguration {
			usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
			format: surface.get_supported_formats(&adapter)[0],
			width: size.width,
			height: size.height,
			present_mode: wgpu::PresentMode::Fifo,
			alpha_mode: CompositeAlphaMode::Auto,
		};
		surface.configure(&device, &config);

		let view_2d = View2D::new(&device, &queue, &config);

		Self {
			surface,
			device,
			queue,
			config,
			size,
			view_2d
		}
	}

	pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
		if new_size.width > 0 && new_size.height > 0 {
			self.size = new_size;
			self.config.width = new_size.width;
			self.config.height = new_size.height;
			self.surface.configure(&self.device, &self.config);
		}
	}

	#[allow(unused_variables)]
	pub(crate) fn input(&mut self, event: &WindowEvent) -> bool {
		false
	}

	pub(crate) fn update(&mut self) {}

	pub(crate) fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
		let output = self.surface.get_current_texture()?;

		let mut encoder = self
			.device
			.create_command_encoder(&wgpu::CommandEncoderDescriptor {
				label: Some("Render Encoder"),
			});

		self.view_2d.draw(&mut encoder, &output);

		self.queue.submit(iter::once(encoder.finish()));
		output.present();

		Ok(())
	}
}
