mod wgpu_utils;

use crate::wgpu_utils::get_surface_config;
use std::sync::Arc;
use wgpu::{Device, InstanceDescriptor, Surface};
use winit::window::Window;

struct RenderManagerState<'a> {
    surface: Surface<'a>,
    device: Device,
}

pub struct RenderManager {
    instance: wgpu::Instance,
    state: Option<RenderManagerState<'static>>,
}

impl RenderManager {
    pub fn set_window(&mut self, window: Arc<Window>) {
        let window_size = window.inner_size();

        let surface = unsafe {
            let target = wgpu::SurfaceTargetUnsafe::from_window(&window).unwrap();
            let s = self.instance.create_surface_unsafe(target).unwrap();
            s
        };

        let adapter = self
            .instance
            .enumerate_adapters(wgpu::Backends::all())
            .into_iter()
            .find(|adapter| adapter.is_surface_supported(&surface))
            .unwrap();

        let (device, queue) = pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                // WebGL doesn't support all of wgpu's features, so if
                // we're building for the web we'll have to disable some.
                required_limits: if cfg!(target_arch = "wasm32") {
                    wgpu::Limits::downlevel_webgl2_defaults()
                } else {
                    wgpu::Limits::default()
                },
                memory_hints: Default::default(),
            },
            None, // Trace path
        ))
        .unwrap();

        let surface_config = get_surface_config(&surface, &adapter, &window_size);

        surface.configure(&device, &surface_config);
        self.state = Some(RenderManagerState { surface, device })
    }
}

pub fn new_render_manager() -> RenderManager {
    let instance = wgpu::Instance::new(InstanceDescriptor {
        backends: wgpu::Backends::all(),
        flags: Default::default(),
        dx12_shader_compiler: Default::default(),
        gles_minor_version: Default::default(),
    });

    RenderManager {
        instance,
        state: None,
    }
}
