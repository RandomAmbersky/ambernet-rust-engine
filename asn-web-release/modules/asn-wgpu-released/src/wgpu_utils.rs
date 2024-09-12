use wgpu::{Adapter, Surface, SurfaceConfiguration};
use winit::dpi::PhysicalSize;

pub fn get_surface_config(
    surface: &Surface,
    adapter: &Adapter,
    size: &PhysicalSize<u32>,
) -> SurfaceConfiguration {
    let surface_caps = surface.get_capabilities(adapter);
    let surface_format = surface_caps
        .formats
        .iter()
        .copied()
        .find(|f| f.is_srgb())
        .unwrap_or(surface_caps.formats[0]);

    // println!("get_config: {:?} ", surface_format);

    let caps_info = surface_caps.formats;
    // println!("caps_info formats: {:?} ", caps_info);

    SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: surface_format,
        width: size.width,
        height: size.height,
        present_mode: surface_caps.present_modes[0],
        desired_maximum_frame_latency: 2,
        alpha_mode: surface_caps.alpha_modes[0],
        view_formats: vec![],
    }
}
