mod asn_window;

use wgpu::Surface;
use crate::library::runner::asn_window::AsnWindow;

pub struct AsnWgpuWinApi {
	window: AsnWindow,
	surface: Surface,
	config: wgpu::SurfaceConfiguration,
	adapter: wgpu::Adapter,
	device: wgpu::Device,
	queue: wgpu::Queue,
}
