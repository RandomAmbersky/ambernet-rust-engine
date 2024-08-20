use crate::errors::AsnError;

pub trait TWinApi {
    type Size2D;
    type FrameContext;

    fn window_resize(&mut self, size: &Self::Size2D);

    fn begin_frame(&mut self) -> Result<Self::FrameContext, AsnError>;
    fn end_frame(&mut self, fcx: Self::FrameContext) -> Result<(), AsnError>;
}
