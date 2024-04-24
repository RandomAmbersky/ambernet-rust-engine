use crate::engine::EngineRealize;
use asn_core::events::{AsnEvent, AsnWindowEvent};
use asn_core::traits::TAsnBaseEngine;

pub fn handle(evt: &AsnEvent, e: &mut EngineRealize) {
    // info!("handle {:?} event", &evt);
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
