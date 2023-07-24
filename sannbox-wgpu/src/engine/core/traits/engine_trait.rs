use crate::engine::core::traits::{TAsnHandler, TAsnWinapi};

pub trait TAsnEngine {
    type WinApi: TAsnWinapi;
    fn get_winapi(&mut self) -> &mut Self::WinApi;
}
