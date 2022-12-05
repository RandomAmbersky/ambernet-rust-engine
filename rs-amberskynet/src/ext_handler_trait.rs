use crate::asn_engine::AsnEngine;

pub trait ExtHandlerTrait {
    fn draw(&self, e: &AsnEngine);
    fn update(&self, e: &AsnEngine);
}
