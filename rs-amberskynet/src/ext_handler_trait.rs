use crate::context::AsnContext;

pub trait ExtHandlerTrait {
    fn draw(&self, e: &AsnContext);
    fn update(&self, e: &AsnContext);
}
