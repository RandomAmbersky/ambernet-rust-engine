use super::super::errors::AsnError;
use super::super::math::Size2D;

pub trait TAsnWinapi {
    type NodeQuad;
    type FrameContext;
    fn window_resize(&mut self, size: &Size2D<u32>);
    // fn redraw(&mut self) -> Option<AsnError>;

    fn begin_frame(&mut self) -> Result<Self::FrameContext, AsnError>;
    fn end_frame(&mut self, fcx: Self::FrameContext) -> Result<(), AsnError>;

    fn new_quad(&mut self) -> Self::NodeQuad;
}
