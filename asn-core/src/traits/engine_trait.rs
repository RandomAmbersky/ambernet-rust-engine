use crate::winapi::TAsnWinapi;

pub trait TAsnBaseEngine {
    fn is_need_exit(&self) -> bool;
    fn set_need_exit(&mut self);
}

pub trait TAsnEngine: TAsnBaseEngine {
    type WinApi: TAsnWinapi;
    fn get_winapi(&mut self) -> &mut Self::WinApi;
}
