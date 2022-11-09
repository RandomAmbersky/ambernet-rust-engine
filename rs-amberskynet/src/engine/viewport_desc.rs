use winit::window::Window;
use crate::engine::viewport::Viewport;

pub struct ViewportDesc {
	window: Window,
	background: wgpu::Color,
	pub(crate) surface: wgpu::Surface,
}

impl ViewportDesc {
	pub(crate) fn new(window: Window, background: wgpu::Color, instance: &wgpu::Instance) -> Self {
		let surface = unsafe { instance.create_surface(&window) };
		Self {
			window,
			background,
			surface,
		}
	}

	pub(crate) fn build(self, adapter: &wgpu::Adapter, device: &wgpu::Device) -> Viewport {
		let size = self.window.inner_size();

		let config = wgpu::SurfaceConfiguration {
			usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
			format: self.surface.get_supported_formats(adapter)[0],
			width: size.width,
			height: size.height,
			present_mode: wgpu::PresentMode::Fifo,
			alpha_mode: self.surface.get_supported_alpha_modes(adapter)[0],
		};

		self.surface.configure(device, &config);

		Viewport { desc: self, config }
	}
}
