mod window;

use crate::window::AsnWindow;
use std::iter;
use wgpu::{Adapter, CommandEncoder, Device, Queue, SurfaceTexture, TextureView};
use winit::event_loop::EventLoop;

mod texture;
pub use texture::AsnTexture;

mod bind_groups;
pub mod defines;
pub mod gfx_error;

use crate::gfx_error::GfxError;
pub use bind_groups::BindGroupEntryBuilder;
pub use bind_groups::BindGroupLayoutBuilder;
use rs_core_gfx::GfxContextTrait;

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
        let instance = wgpu::Instance::new(wgpu::Backends::all());

        let main_window = AsnWindow::new(event_loop, &instance);

        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            compatible_surface: Some(&main_window.surface),
            force_fallback_adapter: false,
            ..Default::default()
        }))
        .expect("Failed to find an appropriate adapter");

        let (device, queue) = pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::downlevel_defaults(),
            },
            None,
        ))
        .expect("Failed to create device");

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
