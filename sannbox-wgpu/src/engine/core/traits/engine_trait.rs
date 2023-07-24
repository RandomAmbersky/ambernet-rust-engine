use crate::engine::core::traits::{TAsnHandler, TAsnWinapi};

pub trait TAsnEngine<H>
where
    H: TAsnHandler,
{
    type WinApi: TAsnWinapi;
    fn init(&mut self);
    fn run(self, h: H);
    fn get_winapi(&mut self) -> &mut Self::WinApi;
}
