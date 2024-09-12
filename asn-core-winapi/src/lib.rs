use asn_core::errors::AsnError;
use asn_core::math::Size2D;
use std::sync::Arc;

pub trait TAsnRenderManager {
    type FrameContext;
    type Window;

    fn window_resize(&mut self, size: Size2D<u32>);

    fn begin_frame(&mut self) -> Result<Self::FrameContext, AsnError>;
    fn end_frame(&mut self, fcx: Self::FrameContext) -> Result<(), AsnError>;
}
