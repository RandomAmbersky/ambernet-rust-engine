pub mod core;
mod winapi;

pub use crate::engine::core::traits::TAsnEngine;
use crate::engine::core::traits::{TAsnBaseEngine, TAsnHandler};
use crate::engine::winapi::{RunnerPreset, WinApi};

pub use crate::engine::winapi::AsnNodeQuad;
pub use crate::engine::winapi::AsnNodeView2d;
pub use crate::engine::winapi::AsnTexture;

pub struct Engine {
    is_need_exit: bool,
    winapi: WinApi,
    preset: Option<RunnerPreset>,
}

impl Engine {
    pub fn new() -> Self {
        let (preset, winapi) = winapi::build();
        Engine {
            is_need_exit: false,
            winapi,
            preset: Some(preset),
        }
    }
    pub fn init(&mut self) {
        println!("Engine:init")
    }
    pub fn run<H: 'static + TAsnHandler<Self>>(mut self, h: H) {
        let preset = self.preset.take().unwrap();
        winapi::run(preset, self, h);
    }
}

impl TAsnBaseEngine for Engine {
    fn is_need_exit(&self) -> bool {
        self.is_need_exit
    }
    fn set_need_exit(&mut self) {
        self.is_need_exit = true
    }
}

impl TAsnEngine for Engine {
    type WinApi = WinApi;
    fn get_winapi(&mut self) -> &mut Self::WinApi {
        &mut self.winapi
    }
}
