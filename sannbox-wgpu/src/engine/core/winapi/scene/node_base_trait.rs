use crate::engine::core::traits::TAsnWinapi;

pub trait TNodeBase {
    type FrameContext;
    type WinApi: TAsnWinapi;
    fn draw(&mut self, fcx: &mut Self::FrameContext);
    fn update(&mut self, gfx: &mut Self::WinApi);
}
