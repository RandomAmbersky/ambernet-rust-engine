mod asn_engine;
mod ext_handler_trait;

pub use asn_engine::AsnEngine;
pub use ext_handler_trait::ExtHandlerTrait;

pub fn new() -> AsnEngine {
    AsnEngine {}
}

pub fn run<E>(e: &AsnEngine, ext: &E)
where
    E: ExtHandlerTrait,
{
    ext.draw(e);
    ext.update(e);
}
