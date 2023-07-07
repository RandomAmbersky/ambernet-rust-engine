use crate::winit_context::WinitContext;
use asn_core::{AsnContext, AsnEvent};
use winit::event::Event;

pub fn process_event(w_ctx: &mut WinitContext, _e: &Event<()>) -> Option<AsnEvent> {
    Some(AsnEvent::Empty)
}
