use crate::amberskynet::asn_engine::AsnEngine;

pub trait ExtCoreTrait {
    fn draw(&self, e: &AsnEngine);
    fn update(&self, e: &AsnEngine);
}
