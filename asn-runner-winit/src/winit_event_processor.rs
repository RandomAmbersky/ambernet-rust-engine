use crate::winit_context::WinitContext;
use asn_core::AsnWindowEvent::{CloseRequested, RedrawRequested, Resized};
use asn_core::{AsnEvent, Size2D};
use winit::event::{Event, WindowEvent};
use winit::window::WindowId;

pub fn process_event(_w_ctx: &mut WinitContext, e: &Event<()>) -> Option<AsnEvent> {
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
        _ => None,
    }
}
