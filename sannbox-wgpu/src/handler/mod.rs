use crate::engine::core::events::{AsnEvent, AsnWindowEvent};
use crate::engine::core::traits::{TAsnBaseEngine, TAsnHandler, TAsnWinapi};
use crate::engine::core::winapi::scene::TNodeBase;
use crate::engine::{Engine, NodeQuad, TAsnEngine};

pub struct Handler {
    quad: NodeQuad,
}

impl Handler {
    pub fn new(e: &mut Engine) -> Self {
        let quad = e.get_winapi().new_quad();
        Handler { quad }
    }
    fn draw(&mut self, e: &mut Engine) {
        let mut fcx = e.get_winapi().begin_frame().unwrap();
        self.quad.draw(&mut fcx);
        e.get_winapi().end_frame(fcx).unwrap();
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
                    self.draw(e);
                }
                AsnWindowEvent::CloseRequested => {
                    e.set_need_exit();
                }
                AsnWindowEvent::RedrawRequested => {
                    self.draw(e);
                }
                _ => {}
            },
            _ => {}
        }
    }
}
