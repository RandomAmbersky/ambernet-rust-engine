use crate::engine::core::events::AsnEvent;
use crate::engine::core::events::AsnWindowEvent::{CloseRequested, RedrawRequested, Resized};
use crate::engine::core::math::Size2D;
use winit::event::{Event, WindowEvent};
use winit::window::WindowId;

pub fn convert_event(e: &Event<()>) -> Option<AsnEvent> {
    // info!("{:?}", e);
    match e {
        Event::RedrawRequested(_window_id) => Some(AsnEvent::WindowEvent(RedrawRequested)),
        Event::WindowEvent {
            ref event,
            window_id,
        } => process_window_event(window_id, event),
        _ => None,
    }
}

fn process_window_event(_window_id: &WindowId, e: &WindowEvent) -> Option<AsnEvent> {
    match e {
        WindowEvent::CloseRequested => Some(AsnEvent::WindowEvent(CloseRequested)),
        WindowEvent::Resized(size) => {
            let w_size: Size2D<u32> = Size2D {
                width: size.width,
                height: size.height,
            };
            Some(AsnEvent::WindowEvent(Resized(w_size)))
        }
        WindowEvent::ScaleFactorChanged {
            new_inner_size: size,
            ..
        } => {
            let w_size: Size2D<u32> = Size2D {
                width: size.width,
                height: size.height,
            };
            Some(AsnEvent::WindowEvent(Resized(w_size)))
        }
        _ => None,
    }
}
