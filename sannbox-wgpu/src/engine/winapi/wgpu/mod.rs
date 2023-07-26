use crate::engine::core::errors::AsnError;
use crate::engine::core::math::Size2D;
use std::iter;

use crate::engine::core::traits::TAsnWinapi;
use crate::engine::winapi::asn_window::AsnWindow;
use crate::engine::winapi::scene::AsnWgpuNodeQuad;
use crate::engine::winapi::NodeQuad;
use wgpu::{
    Adapter, CommandEncoder, Device, Instance, InstanceDescriptor, Queue, Surface, SurfaceTexture,
    TextureView,
};
use winit::event_loop::EventLoop;

pub mod defines;
pub mod texture;

pub struct AsnWgpuWinApi {
    instance: Instance,
    window: AsnWindow,
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
}

impl AsnWgpuWinApi {
    pub fn new(event_loop: &EventLoop<()>) -> Self {
        let instance = wgpu::Instance::new(InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default(),
        });

        let window = AsnWindow::new(event_loop, &instance);
        let adapter = window.get_adapter(&instance);

        let (device, queue) = pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::empty(),
                // WebGL doesn't support all of wgpu's features, so if
                // we're building for the web we'll have to disable some.
                limits: if cfg!(target_arch = "wasm32") {
                    wgpu::Limits::downlevel_webgl2_defaults()
                } else {
                    wgpu::Limits::default()
                },
                label: None,
            },
            None, // Trace path
        ))
        .unwrap();

        window.configure_surface(&adapter, &device);

        AsnWgpuWinApi {
            instance,
            window,
            adapter,
            device,
            queue,
        }
    }

    pub fn get_device(&mut self) -> &Device {
        &self.device
    }
}

pub struct AsnWgpuFrameContext {
    pub frame: SurfaceTexture,
    pub encoder: CommandEncoder,
    pub view: TextureView,
}

impl TAsnWinapi for AsnWgpuWinApi {
    type NodeQuad = NodeQuad;
    type FrameContext = AsnWgpuFrameContext;

    fn window_resize(&mut self, new_size: &Size2D<u32>) {
        println!("{:?}", new_size);
        if new_size.width > 0 && new_size.height > 0 {
            self.window.resize(new_size);
            self.window.configure_surface(&self.adapter, &self.device);
        }
    }

    fn begin_frame(&mut self) -> Result<AsnWgpuFrameContext, AsnError> {
        let frame = self.window.get_current_texture();

        let encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder View2D"),
            });

        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let fcx = AsnWgpuFrameContext {
            encoder,
            frame,
            view,
        };

        Ok(fcx)
    }
    fn end_frame(&mut self, fcx: AsnWgpuFrameContext) -> Result<(), AsnError> {
        self.queue.submit(iter::once(fcx.encoder.finish()));
        fcx.frame.present();
        Ok(())
    }

    fn new_quad(&mut self) -> Self::NodeQuad {
        AsnWgpuNodeQuad::new(self)
    }
}
