use crate::event_converter::convert_window_event_to_asn_event;
use crate::runner_dataset::RunnerDataset;
use asn_core::events::{AsnEvent, AsnEventEmitter, AsnWindowEvent};
use asn_core::traits::{TAsnBaseEngine, TAsnHandler};
use asn_core_winapi::{TAsnRenderManager, TAsnWindowManager};
use asn_logger::{info, trace};
use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::event::{DeviceEvent, DeviceId, WindowEvent};
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowId};

impl<'a, E, H, R> ApplicationHandler for RunnerDataset<'a, E, H, R>
where
    E: TAsnBaseEngine + AsnEventEmitter,
    H: TAsnHandler<E>,
    R: TAsnRenderManager + TAsnWindowManager<Window = Window>,
{
    // This is a common indicator that you can create a window.
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop
            .create_window(Window::default_attributes())
            .unwrap();
        let w = Arc::new(window);
        self.r.init_window(w.clone());
        self.window = Some(w);
    }
    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        // (?)
        if event == WindowEvent::Destroyed {
            event_loop.exit();
            return;
        }

        // Handle window event
        match convert_window_event_to_asn_event(window_id, event) {
            None => {}
            Some(ev) => self.e.emit(ev).unwrap(),
        }
        // info!("window_event: {:?}", event);
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
                None => {
                    // trace!("e.pull() -> None");
                    break;
                }
                Some(evt) => {
                    // trace!("e.pull() -> {:?}", evt);
                    let h = &mut self.h;
                    let e = &mut self.e;
                    h.handle(&evt, e);
                }
            }
        }

        if let Some(window) = self.window.as_ref() {
            window.request_redraw();
            // let evt = AsnEvent::WindowEvent(AsnWindowEvent::RedrawRequested);
            // self.e.emit(evt).unwrap();
            // _window.request_redraw();
            //     self.counter += 1;
        }
    }
}
