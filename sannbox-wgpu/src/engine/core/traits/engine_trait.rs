use crate::engine::core::traits::TAsnWinapi;

pub trait TAsnEngine {
    type WinApi: TAsnWinapi;
    fn get_winapi(&mut self) -> &mut Self::WinApi;
}
