use asn_core::errors::AsnError;
use asn_core::math::Size2D;
use std::sync::Arc;

pub trait TAsnWindowManager {
    type Window;

    fn init_window(&mut self, window: Arc<Self::Window>);
    fn window_resize(&mut self, size: Size2D<u32>);
}

pub trait TAsnRenderManager {
    type FrameContext;

    fn begin_frame(&mut self) -> Result<Self::FrameContext, AsnError>;
    fn end_frame(&mut self, fcx: Self::FrameContext) -> Result<(), AsnError>;
}
