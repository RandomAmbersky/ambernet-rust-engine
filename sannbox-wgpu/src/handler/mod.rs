mod resources;

use std::sync::Arc;
use crate::engine::core::events::{AsnEvent, AsnWindowEvent};
use crate::engine::core::math::{Pos2D, Size2D};
use crate::engine::core::traits::{TAsnBaseEngine, TAsnHandler};
use crate::engine::core::winapi::scene::{TNodeBase, TNodeQuad, TNodeView2d};
use crate::engine::core::winapi::{AsnTextureFormat, TAsnWinapi, TTexture};
use crate::engine::{AsnTexture, Engine, NodeQuad, NodeView2d, TAsnEngine};
use crate::handler::resources::{TEXTURE_QUAD_SOURCE, TEXTURE_TIILES_SOURCE};

pub struct Handler {
    quad: NodeQuad,
    quad2: NodeQuad,
    view: NodeView2d
}

impl Handler {
    pub fn new(e: &mut Engine) -> Self {
        let w = e.get_winapi();
        let mut quad = w.new_quad();
        let mut quad2 = w.new_quad();

        let texture = AsnTexture::from_memory(w, TEXTURE_QUAD_SOURCE, AsnTextureFormat::Rgba8).unwrap();
        let arc_texture = Arc::new(texture);

        quad.set_texture(w, Arc::clone(&arc_texture)).unwrap();
        quad2.set_texture(w, Arc::clone(&arc_texture)).unwrap();

        let mut view = w.new_view2d();
        let tile_texture = AsnTexture::from_memory(w, TEXTURE_TIILES_SOURCE, AsnTextureFormat::Rgba8).unwrap();
        let arc_tile_texture = Arc::new(tile_texture);

        view.set_tile_texture(w, Arc::clone(&arc_tile_texture)).unwrap();
        view.set_view_size(Size2D {
            width: 10,
            height: 10,
        })
        .unwrap();

        Handler { quad, quad2, view  }
    }
    fn draw(&mut self, e: &mut Engine) {
        let mut fcx = e.get_winapi().begin_frame().unwrap();
        // self.quad.draw(&mut fcx);
        // self.quad2.draw(&mut fcx);
        self.view.draw(&mut fcx);
        e.get_winapi().end_frame(fcx).unwrap();
        e.get_winapi().send_event(&AsnEvent::UpdateEvent);
    }
    fn update(&mut self, e: &mut Engine) {
        // self.view.set_cell(&Pos2D { x: 0, y: 0 }, 10).unwrap();
        self.view.update(e.get_winapi())
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
