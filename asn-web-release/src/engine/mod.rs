mod engine;

use asn_core::events::AsnEvent;
use asn_core::traits::TAsnBaseEngine;
pub use engine::Engine as EngineRealize;

pub trait TAsnEngineHandler<E>
where
    E: TAsnBaseEngine,
{
    fn handle(&mut self, evt: &AsnEvent, engine: &mut E);
}
