use crate::engine::core::traits::TAsnWinapi;

pub trait TAsnEngine {
    type WinApi: TAsnWinapi;
    fn init(&mut self);
    fn run(self);
    fn get_winapi(&mut self) -> &mut Self::WinApi;
}
