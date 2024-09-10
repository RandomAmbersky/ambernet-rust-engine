use std::sync::Arc;
use wgpu::{InstanceDescriptor, Surface};
use winit::window::Window;

pub struct RenderManager {
    instance: wgpu::Instance,
    surface: Option<Surface<'static>>,
}

impl RenderManager {
    pub fn set_window(&mut self, window: Arc<Window>) {
        let surface = unsafe {
            let target = wgpu::SurfaceTargetUnsafe::from_window(&window).unwrap();
            let s = self.instance.create_surface_unsafe(target).unwrap();
            s
        };
        self.surface = Some(surface);
    }
}

pub fn new_render_manager() -> RenderManager {
    let instance = wgpu::Instance::new(InstanceDescriptor {
        backends: wgpu::Backends::all(),
        flags: Default::default(),
        dx12_shader_compiler: Default::default(),
        gles_minor_version: Default::default(),
    });

    // let surface = unsafe {
    //     let target = wgpu::SurfaceTargetUnsafe::from_window(&window).unwrap();
    //     let s = instance.create_surface_unsafe(target).unwrap();
    //     s
    // };

    // let adapter = instance
    //       .enumerate_adapters(wgpu::Backends::all())
    //       .into_iter()
    //       .find(|adapter| adapter.is_surface_supported(&surface))
    //       .unwrap()

    RenderManager {
        instance,
        surface: None,
    }
}
