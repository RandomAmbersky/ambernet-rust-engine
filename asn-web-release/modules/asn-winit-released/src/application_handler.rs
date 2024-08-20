use crate::RunnerDataset;
use asn_core::events::{AsnEvent, AsnEventEmitter, AsnWindowEvent};
use asn_core::traits::{TAsnBaseEngine, TAsnHandler};
use asn_logger::{info, trace};
use winit::application::ApplicationHandler;
use winit::event::{DeviceEvent, DeviceId, WindowEvent};
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowId};

impl<'a, E, H> ApplicationHandler for RunnerDataset<'a, E, H>
where
    E: TAsnBaseEngine + AsnEventEmitter,
    H: TAsnHandler<E>,
{
    // This is a common indicator that you can create a window.
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.window = Some(
            event_loop
                .create_window(Window::default_attributes())
                .unwrap(),
        );
    }
    fn window_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        // Handle window event
        // info!("window_event: {:?}", event);
        match event {
            WindowEvent::ActivationTokenDone { .. } => {}
            WindowEvent::Resized(_) => {}
            WindowEvent::Moved(_) => {}
            WindowEvent::CloseRequested => {
                let ev = AsnEvent::WindowEvent(AsnWindowEvent::CloseRequested);
                self.e.emit(ev).unwrap();
            }
            WindowEvent::Destroyed => {}
            WindowEvent::DroppedFile(_) => {}
            WindowEvent::HoveredFile(_) => {}
            WindowEvent::HoveredFileCancelled => {}
            WindowEvent::Focused(_) => {}
            WindowEvent::KeyboardInput { .. } => {}
            WindowEvent::ModifiersChanged(_) => {}
            WindowEvent::Ime(_) => {}
            WindowEvent::CursorMoved { .. } => {}
            WindowEvent::CursorEntered { .. } => {}
            WindowEvent::CursorLeft { .. } => {}
            WindowEvent::MouseWheel { .. } => {}
            WindowEvent::MouseInput { .. } => {}
            WindowEvent::PinchGesture { .. } => {}
            WindowEvent::PanGesture { .. } => {}
            WindowEvent::DoubleTapGesture { .. } => {}
            WindowEvent::RotationGesture { .. } => {}
            WindowEvent::TouchpadPressure { .. } => {}
            WindowEvent::AxisMotion { .. } => {}
            WindowEvent::Touch(_) => {}
            WindowEvent::ScaleFactorChanged { .. } => {}
            WindowEvent::ThemeChanged(_) => {}
            WindowEvent::Occluded(_) => {}
            WindowEvent::RedrawRequested => {
                let evt = AsnEvent::WindowEvent(AsnWindowEvent::RedrawRequested);
                self.e.emit(evt).unwrap();
                // self.window.request_redraw();
            }
        };
    }
    fn device_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _device_id: DeviceId,
        _event: DeviceEvent,
    ) {
        // Handle device event.
    }
    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        if self.e.is_need_exit() {
            let _ = self.window.take();
            event_loop.exit();
            return;
        }

        loop {
            match self.e.pull() {
                None => break,
                Some(evt) => {
                    trace!("e.pull() -> {:?}", evt);
                    let h = &mut self.h;
                    let e = &mut self.e;
                    h.handle(&evt, e);
                }
            }
        }

        if let Some(_window) = self.window.as_ref() {
            let evt = AsnEvent::WindowEvent(AsnWindowEvent::RedrawRequested);
            self.e.emit(evt).unwrap();
            // _window.request_redraw();
            //     self.counter += 1;
        }
    }
}
