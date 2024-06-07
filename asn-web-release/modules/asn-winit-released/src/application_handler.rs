use crate::event_converter::process_window_event;
use crate::RunnerDataset;
use asn_core::traits::{TAsnBaseEngine, TAsnHandler};
use winit::application::ApplicationHandler;
use winit::event::{DeviceEvent, DeviceId, WindowEvent};
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowId};

impl<'a, E, H> ApplicationHandler for RunnerDataset<'a, E, H>
where
    E: TAsnBaseEngine,
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
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        // `unwrap` is fine, the window will always be available when
        // receiving a window event.
        // let window = self.window.as_ref().unwrap();
        // Handle window event.
        match process_window_event(&window_id, &event) {
            None => {}
            Some(e) => {
                self.h.handle(&e, self.e);
            }
        }
    }
    fn device_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        device_id: DeviceId,
        event: DeviceEvent,
    ) {
        // Handle device event.
    }
    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        if self.e.is_need_exit() {
            let _ = self.window.take();
            event_loop.exit()
        }
        if let Some(window) = self.window.as_ref() {
            window.request_redraw();
            self.counter += 1;
        }
    }
}
