use crate::engine::core::traits::{TAsnHandler, TAsnWinapi};

pub trait TAsnEngine {
    type WinApi: TAsnWinapi;
    fn init(&mut self);
    fn run<H: 'static + TAsnHandler>(self, h: H);
    fn get_winapi(&mut self) -> &mut Self::WinApi;
}
