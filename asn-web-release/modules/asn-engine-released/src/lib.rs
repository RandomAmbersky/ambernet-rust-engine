mod engine;

use asn_core::events::AsnEventEmitter;
use asn_core::traits::TAsnBaseEngine;
pub use engine::Engine as EngineRealize;

pub fn get_engine() -> impl TAsnBaseEngine + AsnEventEmitter {
    EngineRealize::new()
}
