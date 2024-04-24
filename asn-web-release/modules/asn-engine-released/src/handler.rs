use asn_core::events::{AsnEvent, AsnWindowEvent};
use asn_core::traits::{TAsnBaseEngine, TAsnHandleEngine};
use asn_logger::trace;

pub fn handle<E>(evt: &AsnEvent, e: &mut E)
where
    E: TAsnBaseEngine + TAsnHandleEngine,
{
    trace!("handle {:?} event", &evt);
    match evt {
        AsnEvent::Empty => {}
        AsnEvent::UpdateEvent => {
            // self.update(e);
        }
        AsnEvent::WindowEvent(w) => match w {
            AsnWindowEvent::CloseRequested => {
                e.set_need_exit();
            }
            AsnWindowEvent::RedrawRequested => {
                // self.draw(e);
            }
            _ => {}
        },
        _ => {}
    }
}
