use crate::errors::AsnError;
use crate::events::AsnEvent;
use crate::math::{Size2D, UnsignedNum};
use crate::winapi::AsnTextureFormat;

#[derive(Default)]
pub struct AsnWinapiConfig {
    pub size: Size2D<u32>,
    pub texture_format: AsnTextureFormat,
}

pub trait TAsnWinapi {
    type FrameContext;
    type Size2D;

    fn get_config(&self) -> &AsnWinapiConfig;

    fn send_event(&mut self, evt: &AsnEvent);

    fn window_resize(&mut self, size: &Self::Size2D);

    fn begin_frame(&mut self) -> Result<Self::FrameContext, AsnError>;
    fn end_frame(&mut self, fcx: Self::FrameContext) -> Result<(), AsnError>;
}
