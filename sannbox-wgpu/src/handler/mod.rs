mod resources;

use crate::engine::core::events::{AsnEvent, AsnWindowEvent};
use crate::engine::core::math::{Array2D, Pos2D, Size2D};
use crate::engine::core::traits::{TAsnBaseEngine, TAsnHandler};
use crate::engine::core::winapi::scene::{TNodeBase, TNodeQuad, TNodeView2d};
use crate::engine::core::winapi::{AsnTextureFormat, TAsnWinapi, TTexture};
use crate::engine::{AsnTexture, Engine, NodeQuad, NodeView2d, TAsnEngine};
use crate::handler::resources::{TEXTURE_QUAD_SOURCE, TEXTURE_TIILES_SOURCE};
use image::GenericImageView;
use rand::prelude::SmallRng;
use rand::{Rng, SeedableRng};
use std::sync::{Arc, Mutex};

const RNG_SEED: u64 = 11;

pub struct Handler {
    arc_texture: Arc<Mutex<AsnTexture>>,
    raw_texture: Array2D<u32, u8>,
    quad: NodeQuad,
    quad2: NodeQuad,
    view: NodeView2d,
    rng: SmallRng,
}

impl Handler {
    pub fn new(e: &mut Engine) -> Self {
        let w = e.get_winapi();
        let mut quad = w.new_quad();
        let mut quad2 = w.new_quad();

        let raw_texture = load_texture(TEXTURE_QUAD_SOURCE);

        let mut texture = AsnTexture::from_raw(
            w,
            &raw_texture.bytes,
            &raw_texture.size,
            AsnTextureFormat::Rgba8,
        )
        .unwrap();
        texture.update_from_raw(w, &raw_texture.bytes).unwrap();

        quad.set_texture(w, &texture).unwrap();
        quad2.set_texture(w, &texture).unwrap();

        let mut view = w.new_view2d();
        let raw_tile_texture = load_texture(TEXTURE_TIILES_SOURCE);
        let tile_texture = AsnTexture::from_raw(
            w,
            &raw_tile_texture.bytes,
            &raw_tile_texture.size,
            AsnTextureFormat::Rgba8,
        )
        .unwrap();

        view.set_tile_texture(w, &tile_texture).unwrap();
        view.set_view_size(&Size2D {
            width: 10,
            height: 10,
        })
        .unwrap();

        let mut rng = SmallRng::seed_from_u64(RNG_SEED);

        Handler {
            rng,
            raw_texture,
            arc_texture: Arc::new(Mutex::new(texture)),
            quad,
            quad2,
            view,
        }
    }
    fn draw(&mut self, e: &mut Engine) {
        // println!("draw");

        let mut fcx = e.get_winapi().begin_frame().unwrap();
        // self.quad.draw(&mut fcx);
        // self.quad2.draw(&mut fcx);
        self.view.draw(&mut fcx);
        e.get_winapi().end_frame(fcx).unwrap();
        e.get_winapi().send_event(&AsnEvent::UpdateEvent);
    }
    fn update(&mut self, e: &mut Engine) {
        let mut rng = self.rng.clone();

        for _ in 0..10 {
            rng = randomize_array(rng, &mut self.raw_texture);
        }

        self.rng = rng;

        let mut texture = self.arc_texture.lock().unwrap();
        texture
            .update_from_raw(e.get_winapi(), &self.raw_texture.bytes)
            .unwrap();

        self.view.set_cell(&Pos2D { x: 5, y: 5 }, 17).unwrap();
        self.view.update(e.get_winapi())
    }
}

impl TAsnHandler<Engine> for Handler {
    fn handle(&mut self, evt: &AsnEvent, e: &mut Engine) {
        // println!("handle {:?} event", &evt);
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

fn load_texture(bytes: &[u8]) -> Array2D<u32, u8> {
    let img = image::load_from_memory(bytes).unwrap();
    let rgba = img.to_rgba8();
    let dimensions = img.dimensions();
    let size = Size2D::<u32> {
        width: dimensions.0,
        height: dimensions.1,
    };
    Array2D {
        size,
        bytes: rgba.to_vec(),
    }
}

fn randomize_array(mut rng: SmallRng, a: &mut Array2D<u32, u8>) -> SmallRng {
    let x = rng.gen_range(0..a.size.width);
    let y = rng.gen_range(0..a.size.height);

    let byte = rng.gen_range(0..3);

    let index = y * a.size.width * 4 + x * 4 + byte;

    let cell: u8 = rng.gen_range(0..255);
    a.bytes[index as usize] = cell;
    // for x in 0..a.size.width {
    //     for y in 0..a.size.height {
    //         let index = ((y * a.size.width + x) * 4) as usize;
    //         a.bytes[index] = rng.gen_range(0..128);
    //         a.bytes[index+1] = rng.gen_range(0..128);
    //         a.bytes[index+2] = rng.gen_range(0..128);
    //         a.bytes[index+3] = rng.gen_range(0..128);
    //     }
    // }
    rng
}
