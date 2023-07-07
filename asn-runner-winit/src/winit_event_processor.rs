use asn_core::{AsnContext, AsnEvent};
use winit::event::Event;

pub fn process_event(_ctx: &mut AsnContext, _e: &Event<()>) -> Option<AsnEvent> {
    Some(AsnEvent::Empty)
}
