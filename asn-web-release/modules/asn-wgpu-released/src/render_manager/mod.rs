use crate::wgpu_utils::get_surface_config;
use asn_core::errors::AsnError;
use asn_core::math::Size2D;
use asn_core_winapi::TAsnRenderManager;
use std::iter;
use std::sync::Arc;
use wgpu::{Device, Queue, Surface};

pub struct AsnWgpuFrameContext {
    pub frame: wgpu::SurfaceTexture,
    pub encoder: wgpu::CommandEncoder,
    pub view: wgpu::TextureView,
}

struct RenderManagerState<'a> {
    surface: Surface<'a>,
    device: Device,
    queue: Queue,
}

pub struct RenderManager {
    instance: wgpu::Instance,
    state: Option<RenderManagerState<'static>>,
}

impl TAsnRenderManager for RenderManager {
    type FrameContext = AsnWgpuFrameContext;
    type Window = winit::window::Window;

    fn set_window(&mut self, window: Arc<Self::Window>) {
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
        self.state = Some(RenderManagerState {
            surface,
            device,
            queue,
        })
    }

    fn window_resize(&mut self, size: Size2D<u32>) {
        todo!()
    }

    fn begin_frame(&mut self) -> Result<Self::FrameContext, AsnError> {
        let state = self.state.as_mut().unwrap();
        let frame = state.surface.get_current_texture().unwrap();

        let texture_format = frame.texture.format();

        let encoder = state
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder View2D"),
            });

        let texture_view_descriptor = wgpu::TextureViewDescriptor {
            // format: Some(TextureFormat::Rgba8UnormSrgb),
            // format: Some(texture_format),
            ..Default::default()
        };

        let view = frame.texture.create_view(&texture_view_descriptor);

        let fcx = AsnWgpuFrameContext {
            encoder,
            frame,
            view,
        };

        Ok(fcx)
    }

    fn end_frame(&mut self, fcx: Self::FrameContext) -> Result<(), AsnError> {
        let state = self.state.as_mut().unwrap();
        state.queue.submit(iter::once(fcx.encoder.finish()));
        fcx.frame.present();
        Ok(())
    }
}
