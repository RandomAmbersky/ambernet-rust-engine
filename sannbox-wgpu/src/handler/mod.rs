use crate::engine::core::events::{AsnEvent, AsnWindowEvent};
use crate::engine::core::traits::{TAsnHandler, TAsnWinapi};
use crate::engine::{Engine, TAsnEngine};

pub struct Handler {}

impl Handler {
    pub fn new() -> Self {
        Handler {}
    }
}

impl TAsnHandler<Engine> for Handler {
    fn handle(&mut self, evt: &AsnEvent, e: &mut Engine) {
        println!("handle {:?} event", &evt);
        match evt {
            AsnEvent::Empty => {}
            AsnEvent::WindowEvent(w) => match w {
                AsnWindowEvent::Resized(size) => {
                    e.get_winapi().window_resize(size);
                    e.get_winapi().redraw();
                }
                AsnWindowEvent::RedrawRequested => {}
                AsnWindowEvent::CloseRequested => {
                    // ctx.set_need_exit();
                }
            },
        }
    }
}
