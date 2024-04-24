use std::iter;

use crate::engine::asn_winapi::asn_window::AsnWindow;
use crate::engine::asn_winapi::event_converter::{convert_asn_event, CustomEvent};
use crate::engine::asn_winapi::utils::ToWgpuFormat;
use asn_core::errors::AsnError;
use asn_core::events::AsnEvent;
use asn_core::math::Size2D;
use asn_core::winapi::{AsnTextureFormat, AsnWinapiConfig, TAsnWinapi};
use asn_logger::{info, trace};
use wgpu::{Instance, InstanceDescriptor};
use winit::event_loop::{EventLoop, EventLoopProxy};

pub mod bind_groups;
pub mod texture;

#[allow(dead_code)]
pub struct AsnWgpuWinApi {
    config: AsnWinapiConfig,
    instance: Instance,
    window: AsnWindow,
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
    event_loop_proxy: EventLoopProxy<CustomEvent>,
}

//  WGPU-specific function for internal wgpu node purpouse
impl AsnWgpuWinApi {
    pub fn new(event_loop: &EventLoop<CustomEvent>) -> Self {
        trace!("AsnWgpuWinApi:new");
        let size = Size2D {
            width: 1024_u32,
            height: 768_u32,
        };

        let mut config = AsnWinapiConfig {
            size,
            ..Default::default()
        };

        let instance = wgpu::Instance::new(InstanceDescriptor {
            backends: wgpu::Backends::all(),
            flags: Default::default(),
            dx12_shader_compiler: Default::default(),
            gles_minor_version: Default::default(),
        });

        let window = AsnWindow::new(event_loop, &instance, &config.size);

        let adapter = window.get_adapter(&instance);

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
            },
            None, // Trace path
        ))
        .unwrap();

        window.configure_surface(&adapter, &device);
        let surface_texture_format = window.get_current_texture().texture.format();
        config.texture_format = AsnTextureFormat::get_from(surface_texture_format);

        let event_loop_proxy = event_loop.create_proxy();

        AsnWgpuWinApi {
            event_loop_proxy,
            config,
            instance,
            window,
            adapter,
            device,
            queue,
        }
    }
    pub fn get_device(&mut self) -> &wgpu::Device {
        &self.device
    }

    pub fn write_buffer(
        &mut self,
        buffer: &wgpu::Buffer,
        offset: wgpu::BufferAddress,
        data: &[u8],
    ) {
        self.queue.write_buffer(buffer, offset, data);
    }
}

pub struct AsnWgpuFrameContext {
    pub frame: wgpu::SurfaceTexture,
    pub encoder: wgpu::CommandEncoder,
    pub view: wgpu::TextureView,
}

impl TAsnWinapi for AsnWgpuWinApi {
    type FrameContext = AsnWgpuFrameContext;
    type Size2D = Size2D<u32>;

    fn get_config(&self) -> &AsnWinapiConfig {
        &self.config
    }

    fn send_event(&mut self, evt: &AsnEvent) {
        let e = convert_asn_event(evt);
        self.event_loop_proxy.send_event(e).ok();
    }

    fn window_resize(&mut self, new_size: &Self::Size2D) {
        // info!("window_resize {:?}", new_size);
        if new_size.width > 0 && new_size.height > 0 {
            self.config.size = new_size.clone();
            self.window.resize(new_size);
            self.window.configure_surface(&self.adapter, &self.device);
            let surface_texture_format = self.window.get_current_texture().texture.format();
            self.config.texture_format = AsnTextureFormat::get_from(surface_texture_format);
        }
    }

    fn begin_frame(&mut self) -> Result<AsnWgpuFrameContext, AsnError> {
        let frame = self.window.get_current_texture();

        let texture_format = frame.texture.format();

        let encoder = self
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
    fn end_frame(&mut self, fcx: AsnWgpuFrameContext) -> Result<(), AsnError> {
        self.queue.submit(iter::once(fcx.encoder.finish()));
        fcx.frame.present();
        Ok(())
    }
}
