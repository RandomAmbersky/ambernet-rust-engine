use crate::context::AsnContext;

pub trait ExtHandlerTrait {
    fn draw(&mut self, e: &mut AsnContext);
    fn update(&mut self, e: &mut AsnContext);
}
