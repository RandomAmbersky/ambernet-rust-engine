use asn_core::events::{AsnEvent, AsnWindowEvent};
use asn_core::traits::{TAsnBaseEngine, TAsnHandler};
use asn_logger::trace;

#[derive(Default, Debug)]
pub struct Handler {}

impl Handler {
    pub fn new() -> Self {
        trace!("Handler:new");
        Self::default()
    }
}

impl<E> TAsnHandler<E> for Handler
where
    E: TAsnBaseEngine,
{
    fn handle(&mut self, evt: &AsnEvent, engine: &mut E) {
        handle(evt, engine)
    }
}

fn handle<E>(evt: &AsnEvent, e: &mut E)
where
    E: TAsnBaseEngine,
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
