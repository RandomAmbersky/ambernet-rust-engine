use super::super::errors::AsnError;
use super::super::math::Size2D;

pub trait TAsnWinapi {
    type NodeQuad;
    fn window_resize(&mut self, size: &Size2D<u32>);
    fn redraw(&mut self) -> Option<AsnError>;
    fn new_quad(&mut self) -> Self::NodeQuad;
}
