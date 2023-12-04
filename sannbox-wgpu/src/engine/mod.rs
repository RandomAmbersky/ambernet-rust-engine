mod winapi;

use crate::engine::winapi::{RunnerPreset, WinApi};
use asn_core::events::AsnEvent;
use asn_core::events::AsnWindowEvent::Resized;
use asn_core::math::Size2D;
use asn_core::traits::{TAsnBaseEngine, TAsnEngine, TAsnHandler};
use asn_core::winapi::TAsnWinapi;

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
        // println!("Engine:init")
    }
    pub fn run<H: 'static + TAsnHandler<Self>>(mut self, h: H) {
        let preset = self.preset.take().unwrap();
        winapi::run(preset, self, h).unwrap();
    }
}

impl Engine {
    pub fn resize(&mut self) {
        let size = Size2D {
            width: 100,
            height: 100,
        };
        let e = AsnEvent::WindowEvent(Resized(size));
        self.get_winapi().send_event(&e);
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
