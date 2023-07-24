mod core;
mod event_runner;

pub use crate::engine::core::traits::TAsnEngine;
use crate::engine::event_runner::{RunnerPreset, WinApi};

pub struct Engine {
    winapi: WinApi,
    preset: Option<RunnerPreset>,
}

impl Engine {
    pub fn new() -> Self {
        let (preset, winapi) = event_runner::build();
        Engine {
            winapi,
            preset: Some(preset),
        }
    }
}

impl TAsnEngine for Engine {
    type WinApi = WinApi;

    fn init(&mut self) {
        println!("Engine:init")
    }
    fn run(mut self) {
        let preset = self.preset.take().unwrap();
        event_runner::run(preset, self);
    }
    fn get_winapi(&mut self) -> &mut Self::WinApi {
        &mut self.winapi
    }
}
