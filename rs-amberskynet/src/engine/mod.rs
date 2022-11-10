mod asn_engine_state;
pub mod state;
mod viewport;
mod viewport_desc;
mod window;

use crate::engine::state::AsnState;
use crate::view_2d::View2D;
use log::{debug, error};
use wgpu::{Device, Queue, SurfaceTexture, TextureFormat};
use winit::dpi::PhysicalSize;
use winit::event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget};
use winit::window::WindowId;

pub struct AsnEngine {
    state: AsnState,
    view_2d: View2D,
}

impl AsnEngine {
    pub async fn new(event_loop: &EventLoop<()>) -> Self {
        // let instance = wgpu::Instance::new(wgpu::Backends::all());

        let state = AsnState::new(event_loop).await;

        // let window = WindowBuilder::new().build(event_loop).unwrap();

        // let window_color = wgpu::Color {
        //     r: 0.5,
        //     g: 0.5,
        //     b: 0.5,
        //     a: 1.0,
        // };

        // let viewport_desc = ViewportDesc::new(window, window_color, &instance);

        // let adapter = instance
        //     .request_adapter(&wgpu::RequestAdapterOptions {
        //         Request an adapter which can render to our surface
        // compatible_surface: Some(&viewport_desc.surface),
        // force_fallback_adapter: false,
        // ..Default::default()
        // })
        // .await
        // .expect("Failed to find an appropriate adapter");

        // let (device, queue) = adapter
        //     .request_device(
        //         &wgpu::DeviceDescriptor {
        //             label: None,
        //             features: wgpu::Features::empty(),
        //             limits: wgpu::Limits::downlevel_defaults(),
        //         },
        //         None,
        //     )
        //     .await
        //     .expect("Failed to create device");

        // let viewport = viewport_desc.build(&adapter, &device);

        // let texture_format = TextureFormat::Rgba32Float;
        let texture_format = state.main_window.get_supported_format(&state.adapter);

        let view_2d = View2D::new(&state.device, &state.queue, texture_format);

        AsnEngine { state, view_2d }
    }

    pub fn process_event(
        &mut self,
        event: &Event<()>,
        _event_loop_window_target: &EventLoopWindowTarget<()>,
        control_flow: &mut ControlFlow,
    ) {
        match event {
            Event::WindowEvent { window_id, event } => {
                self.process_window_event(control_flow, window_id, event)
            }
            Event::RedrawRequested(window_id) => {
                self.process_redraw_requested(window_id, control_flow)
            }
            Event::MainEventsCleared {} => self.process_main_events_cleared(),
            Event::RedrawEventsCleared {} => {}
            Event::NewEvents(..) => {}
            _ => {
                // debug!("unprocesseble event {:?}", event);
            }
        }
    }
}

impl AsnEngine {
    fn process_window_event(
        &mut self,
        control_flow: &mut ControlFlow,
        window_id: &WindowId,
        event: &WindowEvent,
    ) {
        if window_id != &self.state.main_window.get_windows_id() {
            error!("not correct window_id: {:?}", window_id);
            return;
        }
        match event {
            WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
            WindowEvent::ScaleFactorChanged {
                scale_factor: _,
                new_inner_size,
            } => self.process_resized(new_inner_size),
            WindowEvent::CursorMoved { .. } => {}
            WindowEvent::CursorLeft { .. } => {}
            WindowEvent::CursorEntered { .. } => {}
            WindowEvent::Resized(size) => {
                self.process_resized(size);
            }
            WindowEvent::KeyboardInput {
                device_id: _,
                input,
                is_synthetic: _,
            } => self.process_keyboard_event(input, control_flow),
            _ => {
                debug!("unprocesseble window event {:?}", event);
            }
        }
        // debug!("this is a debug {:?}", event);
    }
}

impl AsnEngine {
    fn process_redraw_requested(&mut self, window_id: &WindowId, control_flow: &mut ControlFlow) {
        if window_id != &self.state.main_window.get_windows_id() {
            error!("not correct window_id: {:?}", window_id);
        }
        match self.render() {
            Ok(_) => {}
            // Reconfigure the surface if it's lost or outdated
            Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                // let size =
                //     PhysicalSize::new(self.viewport.config.width, self.viewport.config.height);
                // self.viewport.resize(&self.device, size);
            }
            // The system is out of memory, we should probably quit
            Err(wgpu::SurfaceError::OutOfMemory) => {
                log::warn!("OutOfMemory");
                *control_flow = ControlFlow::Exit;
            }
            // We're ignoring timeouts
            Err(wgpu::SurfaceError::Timeout) => log::warn!("Timeout"),
        }
    }
}

impl AsnEngine {
    fn process_main_events_cleared(&mut self) {
        self.state.main_window.request_redraw();
    }
}

impl AsnEngine {
    fn process_keyboard_event(&self, input: &KeyboardInput, control_flow: &mut ControlFlow) {
        if let KeyboardInput {
            state: ElementState::Pressed,
            virtual_keycode: Some(VirtualKeyCode::Escape),
            ..
        } = input
        {
            *control_flow = ControlFlow::Exit
        }
    }
}

impl AsnEngine {
    fn process_resized(&mut self, size: &PhysicalSize<u32>) {
        debug!("resize window {:?}", size);
        self.state.main_window.resize(&self.state.device, *size);
        self.state.main_window.request_redraw();
    }
}

impl AsnEngine {
    // fn render_empty(&mut self, frame: &SurfaceTexture) -> Result<(), wgpu::SurfaceError> {
    //     let view = frame
    //         .texture
    //         .create_view(&wgpu::TextureViewDescriptor::default());
    //     let mut encoder = self
    //         .state
    //         .device
    //         .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
    //     {
    //         let _rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
    //             label: None,
    //             color_attachments: &[Some(wgpu::RenderPassColorAttachment {
    //                 view: &view,
    //                 resolve_target: None,
    //                 ops: wgpu::Operations {
    //                     load: wgpu::LoadOp::Clear(self.viewport.desc.background),
    //                     store: true,
    //                 },
    //             })],
    //             depth_stencil_attachment: None,
    //         });
    //     }
    //     self.state.queue.submit(Some(encoder.finish()));
    //     Ok(())
    // }
    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let frame = self.state.main_window.get_current_texture();
        // self.render_empty(&frame).expect("render_empty panic");
        self.view_2d
            .draw(&self.state.device, &self.state.queue, &frame);
        frame.present();
        Ok(())
    }
}
