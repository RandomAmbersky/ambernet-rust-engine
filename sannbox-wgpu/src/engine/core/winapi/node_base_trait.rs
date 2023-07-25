use crate::engine::core::traits::TAsnWinapi;

pub trait TNodeBase {
    type WinApi: TAsnWinapi;
    fn draw(&mut self, gfx: &mut Self::WinApi);
}
