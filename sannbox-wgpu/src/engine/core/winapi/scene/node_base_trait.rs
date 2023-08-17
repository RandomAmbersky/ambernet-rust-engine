use crate::engine::core::winapi::{TAsnWinapi, TTexture};

pub trait TNodeBase {
    type FrameContext;
    type WinApi: TAsnWinapi;
    type AsnTexture: TTexture;
    fn draw(&mut self, fcx: &mut Self::FrameContext);
    fn update(&mut self, gfx: &mut Self::WinApi);
}
