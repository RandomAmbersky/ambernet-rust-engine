use super::super::errors::AsnError;
use super::super::math::Size2D;
use crate::engine::core::winapi::AsnTextureFormat;

#[derive(Default)]
pub struct AsnWinapiConfig {
    pub size: Size2D<u32>,
    pub texture_format: AsnTextureFormat,
}

pub trait TAsnWinapi {
    type NodeQuad;
    type NodeView2d;

    type FrameContext;

    fn get_config(&self) -> &AsnWinapiConfig;

    fn window_resize(&mut self, size: &Size2D<u32>);
    fn begin_frame(&mut self) -> Result<Self::FrameContext, AsnError>;
    fn end_frame(&mut self, fcx: Self::FrameContext) -> Result<(), AsnError>;

    fn new_quad(&mut self) -> Self::NodeQuad;
    fn new_view2d(&mut self) -> Self::NodeView2d;
}
