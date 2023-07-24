pub mod core;
mod event_runner;

pub use crate::engine::core::traits::TAsnEngine;
use crate::engine::core::traits::{TAsnBaseEngine, TAsnHandler};
use crate::engine::event_runner::{RunnerPreset, WinApi};

pub struct Engine {
    is_need_exit: bool,
    winapi: WinApi,
    preset: Option<RunnerPreset>,
}

impl Engine {
    pub fn new() -> Self {
        let (preset, winapi) = event_runner::build();
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
        event_runner::run(preset, self, h);
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
