mod window;

use crate::window::AsnWindow;
use std::iter;
use wgpu::{
    Adapter, CommandEncoder, Device, InstanceDescriptor, Queue, SurfaceTexture, TextureView,
};
use winit::event_loop::EventLoop;

mod texture;
pub use texture::AsnTexture;

mod bind_groups;
pub mod defines;
pub mod gfx_error;

use crate::gfx_error::GfxError;
pub use bind_groups::BindGroupEntryBuilder;
pub use bind_groups::BindGroupLayoutBuilder;
use rs_gfx_core::GfxContextTrait;

pub struct AsnGfx {
    pub main_window: AsnWindow,
    pub device: Device,
    pub adapter: Adapter,
    pub queue: Queue,
    pub fcx: Option<FrameCtx>,
}

pub struct FrameCtx {
    pub encoder: CommandEncoder,
    frame: SurfaceTexture,
    pub view: TextureView,
}

impl AsnGfx {
    pub fn new(event_loop: &EventLoop<()>) -> Self {
        let instance = wgpu::Instance::new(InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default(),
        });

        // let report = instance.generate_report();
        // println!("{:?}", report);

        // instance.create_surface_from_surface_handle();
        let main_window = AsnWindow::new(event_loop, &instance);

        let adapter = instance
            .enumerate_adapters(wgpu::Backends::all())
            .find(|adapter| adapter.is_surface_supported(&main_window.surface))
            .unwrap();
        // let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
        //     compatible_surface: Some(&main_window.surface),
        //     force_fallback_adapter: false,
        //     ..Default::default()
        // }))
        // .expect("Failed to find an appropriate adapter");

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

        // let (device, queue) = pollster::block_on(adapter.request_device(
        //     &wgpu::DeviceDescriptor {
        //         label: None,
        //         features: wgpu::Features::empty(),
        //         limits: wgpu::Limits::downlevel_defaults(),
        //     },
        //     None,
        // ))
        // .expect("Failed to create device");

        AsnGfx {
            main_window,
            device,
            adapter,
            queue,
            fcx: None,
        }
    }
}

impl GfxContextTrait<GfxError> for AsnGfx {
    fn begin_frame(&mut self) -> Result<(), GfxError> {
        let frame = self.main_window.get_current_texture();

        let encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder View2D"),
            });

        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        self.fcx = Some(FrameCtx {
            encoder,
            frame,
            view,
        });
        Ok(())
    }
    fn end_frame(&mut self) -> Result<(), GfxError> {
        if let Some(fcx) = self.fcx.take() {
            self.queue.submit(iter::once(fcx.encoder.finish()));
            fcx.frame.present();
        }
        Ok(())
    }
}
