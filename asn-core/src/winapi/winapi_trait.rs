use crate::errors::AsnError;
use crate::events::AsnEvent;
use crate::math::Size2D;
use crate::winapi::AsnTextureFormat;

#[derive(Default)]
pub struct AsnWinapiConfig {
    pub size: Size2D<u32>,
    pub texture_format: AsnTextureFormat,
}

pub trait TAsnWinapi {
    type FrameContext;

    fn get_config(&self) -> &AsnWinapiConfig;

    fn send_event(&mut self, evt: &AsnEvent);

    fn window_resize(&mut self, size: &Size2D<u32>);

    fn begin_frame(&mut self) -> Result<Self::FrameContext, AsnError>;
    fn end_frame(&mut self, fcx: Self::FrameContext) -> Result<(), AsnError>;
}
