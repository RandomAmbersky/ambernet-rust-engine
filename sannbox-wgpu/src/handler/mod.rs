mod resources;

use crate::engine::core::events::{AsnEvent, AsnWindowEvent};
use crate::engine::core::traits::{TAsnBaseEngine, TAsnHandler, TAsnWinapi};
use crate::engine::core::winapi::scene::{TNodeBase, TNodeQuad, TNodeView2d};
use crate::engine::core::winapi::AsnTextureFormat;
use crate::engine::{Engine, NodeQuad, NodeView2d, TAsnEngine};
use crate::handler::resources::{TEXTURE_QUAD_SOURCE, TEXTURE_TIILES_SOURCE};

pub struct Handler {
    quad: NodeQuad,
    view: NodeView2d,
}

impl Handler {
    pub fn new(e: &mut Engine) -> Self {
        let w = e.get_winapi();
        let mut quad = w.new_quad();
        let mut view = w.new_view2d();
        quad.set_texture(w, TEXTURE_QUAD_SOURCE, AsnTextureFormat::Rgba8)
            .unwrap();
        view.set_tile_texture(w, TEXTURE_TIILES_SOURCE, AsnTextureFormat::Rgba8)
            .unwrap();
        Handler { quad, view }
    }
    fn draw(&mut self, e: &mut Engine) {
        let mut fcx = e.get_winapi().begin_frame().unwrap();
        // self.quad.draw(&mut fcx);
        self.view.draw(&mut fcx);
        e.get_winapi().end_frame(fcx).unwrap();
    }
    fn update(&mut self, e: &mut Engine) {
        println!("Handler update");
    }
}

impl TAsnHandler<Engine> for Handler {
    fn handle(&mut self, evt: &AsnEvent, e: &mut Engine) {
        println!("handle {:?} event", &evt);
        match evt {
            AsnEvent::Empty => {}
            AsnEvent::UpdateEvent => {
                self.update(e);
            }
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
