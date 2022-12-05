use crate::amberskynet::asn_engine::AsnEngine;
use crate::amberskynet::core::ext_core_trait::ExtCoreTrait;

pub struct ExternalCore {}

impl ExternalCore {
    pub fn new() -> ExternalCore {
        Self {}
    }
}

impl ExtCoreTrait for ExternalCore {
    fn draw(&self, e: &AsnEngine) {
        // todo!()
    }

    fn update(&self, e: &AsnEngine) {
        // todo!()
    }
}
