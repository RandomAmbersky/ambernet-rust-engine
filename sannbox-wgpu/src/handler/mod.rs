use crate::engine::core::events::{AsnEvent, AsnWindowEvent};
use crate::engine::core::traits::{TAsnHandler, TAsnWinapi};
use crate::engine::core::winapi::TNodeBase;
use crate::engine::{Engine, NodeQuad, TAsnEngine};

pub struct Handler {
    quad: NodeQuad,
}

impl Handler {
    pub fn new(e: &mut Engine) -> Self {
        let mut quad = e.get_winapi().new_quad();
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
            AsnEvent::WindowEvent(w) => match w {
                AsnWindowEvent::RedrawRequested => {
                    self.draw(e);
                    // self.quad.draw(e.get_winapi());
                    // e.get_winapi().redraw();
                }
                _ => {}
            },
            _ => {}
        }
    }
}
