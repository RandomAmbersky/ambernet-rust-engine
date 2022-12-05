use crate::amberskynet::asn_engine::AsnEngine;
use crate::amberskynet::core::ext_core_trait::ExtCoreTrait;

pub mod asn_engine;
pub mod core;

pub fn new() -> AsnEngine {
    AsnEngine {}
}

pub fn run<E>(e: &AsnEngine, ext: &E)
where
    E: ExtCoreTrait,
{
    ext.draw(e);
    ext.update(e);
}
