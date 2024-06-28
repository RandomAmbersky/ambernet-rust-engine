use asn_core::events::{AsnEvent, AsnWindowEvent};
use asn_core::traits::TAsnBaseEngine;
use asn_logger::trace;

pub fn handle<E>(evt: &AsnEvent, e: &mut E)
where
    E: TAsnBaseEngine,
{
    trace!("handle {:?} event", &evt);
    match evt {
        AsnEvent::Empty => {}
        AsnEvent::UpdateEvent => {}
        AsnEvent::WindowEvent(w) => handle_window_event(w, e),
        _ => {}
    }
}

pub fn handle_window_event<E>(w: &AsnWindowEvent, e: &mut E)
where
    E: TAsnBaseEngine,
{
    match w {
        AsnWindowEvent::CloseRequested => {
            e.set_need_exit();
        }
        AsnWindowEvent::RedrawRequested => {
            // self.draw(e);
        }
        _ => {}
    }
}
