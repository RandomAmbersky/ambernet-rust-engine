use crate::winit_context::WinitContext;
use asn_core::AsnWindowEvent::CloseRequested;
use asn_core::{AsnEvent, AsnWindowId};
use winit::event::{Event, WindowEvent};
use winit::window::WindowId;

pub fn process_event(_w_ctx: &mut WinitContext, e: &Event<()>) -> Option<AsnEvent> {
    match e {
        Event::WindowEvent {
            ref event,
            window_id,
        } => process_window_event(window_id, event),
        _ => None,
    }
}

fn process_window_event(window_id: &WindowId, e: &WindowEvent) -> Option<AsnEvent> {
    let id: u64 = u64::from(*window_id);
    match e {
        WindowEvent::CloseRequested => Some(AsnEvent::WindowEvent {
            window_id: AsnWindowId::from(id),
            event: CloseRequested,
        }),
        _ => None,
    }
}
