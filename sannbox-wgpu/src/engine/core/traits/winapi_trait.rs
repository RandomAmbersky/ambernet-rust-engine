use super::super::errors::AsnError;
use super::super::math::Size2D;

pub trait AsnWinapiTrait {
    fn window_resize(&mut self, size: &Size2D<u32>);
    fn redraw(&mut self) -> Option<AsnError>;
}
