use crate::winit_context::WinitContext;
use asn_core::AsnEvent;
use winit::event::{Event, WindowEvent};
use winit::window::WindowId;

pub fn process_event(_w_ctx: &mut WinitContext, e: &Event<()>) -> Option<AsnEvent> {
    match e {
        Event::WindowEvent {
            ref event,
            window_id,
        } => process_window_event(window_id, event),
        _ => Some(AsnEvent::Empty),
    }
}

fn process_window_event(_window_id: &WindowId, e: &WindowEvent) -> Option<AsnEvent> {
    match e {
        WindowEvent::CloseRequested => Some(AsnEvent::Empty),
        _ => None,
    }
}
