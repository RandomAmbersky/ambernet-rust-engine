mod resources;

use crate::engine::{AsnNodeQuad, AsnNodeView2d, AsnTexture, Engine};
use crate::handler::resources::{MAP_TMX, TEXTURE_QUAD_SOURCE, TEXTURE_TIILES_SOURCE};
use asn_core::events::{AsnEvent, AsnWindowEvent};
use asn_core::math::{Array2D, Pos2D, Size2D};
use asn_core::traits::{TAsnBaseEngine, TAsnEngine, TAsnHandler};
use asn_core::winapi::scene::{TNodeBase, TNodeQuad, TNodeView2d};
use asn_core::winapi::{AsnTextureFormat, TAsnWinapi, TTexture};
use asn_image::load_texture;
use asn_tiled::load_tmx;
use rand::prelude::SmallRng;
use rand::{Rng, SeedableRng};
use std::sync::{Arc, Mutex};

const RNG_SEED: u64 = 11;

const MAP_VIEW_SIZE: Size2D<u32> = Size2D {
    width: 32,
    height: 32,
};

const TILE_SIZE: Size2D<u32> = Size2D {
    width: 16,
    height: 16,
};

pub struct Handler {
    arc_texture: Arc<Mutex<AsnTexture>>,
    raw_texture: Array2D<u32, u8>,
    quad: AsnNodeQuad,
    quad2: AsnNodeQuad,
    view: AsnNodeView2d,
    rng: SmallRng,
}

impl Handler {
    pub fn new(e: &mut Engine) -> Self {
        let w = e.get_winapi();
        let mut quad = AsnNodeQuad::new(w);
        let mut quad2 = AsnNodeQuad::new(w);

        let map = load_tmx(MAP_TMX).unwrap();
        println!("{:?} ", map);

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

        let raw_tile_texture = load_texture(TEXTURE_TIILES_SOURCE);
        let tile_texture = AsnTexture::from_raw(
            w,
            &raw_tile_texture.bytes,
            &raw_tile_texture.size,
            AsnTextureFormat::Rgba8UnormSrgb,
        )
        .unwrap();
        let mut view = AsnNodeView2d::new(w, &tile_texture, &MAP_VIEW_SIZE, &TILE_SIZE);
        // fill_by_index(&mut view);
        view.update_from_raw(&map.layers[0].bytes).unwrap();

        let rng = SmallRng::seed_from_u64(RNG_SEED);

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

        // for _ in 0..10 {
        //     rng = randomize_array(rng, &mut self.raw_texture);
        // }
        //

        // let mut texture = self.arc_texture.lock().unwrap();
        // texture
        //     .update_from_raw(e.get_winapi(), &self.raw_texture.bytes)
        //     .unwrap();

        // self.view.set_cell(&Pos2D { x: 0, y: 0 }, 1).unwrap();
        // self.view.set_cell(&Pos2D { x: 1, y: 1 }, 16).unwrap();
        // self.view.set_cell(&Pos2D { x: 2, y: 2 }, 32).unwrap();

        // for _ in 0..1000 {
        //     rng = randomize_view_cell(rng, &mut self.view);
        // }

        self.view.update(e.get_winapi());
        self.rng = rng;
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
                    self.view.set_screen_size(size);
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

fn randomize_view_cell(mut rng: SmallRng, v: &mut AsnNodeView2d) -> SmallRng {
    let x: u32 = rng.gen_range(0..MAP_VIEW_SIZE.width);
    let y: u32 = rng.gen_range(0..MAP_VIEW_SIZE.height);
    let c: u8 = rng.gen_range(0..128);
    v.set_cell(&Pos2D { x, y }, c).unwrap();
    rng
}

fn fill_by_index(v: &mut AsnNodeView2d) {
    let mut index: u8 = 0;
    for y in 0..MAP_VIEW_SIZE.height {
        for x in 0..MAP_VIEW_SIZE.width {
            v.set_cell(&Pos2D { x, y }, index).unwrap();
            index = index.wrapping_add(1);
        }
    }
}

fn randomize_view(mut rng: SmallRng, v: &mut AsnNodeView2d) -> SmallRng {
    for x in 0..MAP_VIEW_SIZE.width {
        for y in 0..MAP_VIEW_SIZE.height {
            let c: u8 = rng.gen_range(0..128);
            v.set_cell(&Pos2D { x, y }, c).unwrap();
        }
    }
    rng
}

fn randomize_array(mut rng: SmallRng, a: &mut Array2D<u32, u8>) -> SmallRng {
    let x = rng.gen_range(0..a.size.width);
    let y = rng.gen_range(0..a.size.height);

    let byte = rng.gen_range(0..3);

    let index = y * a.size.width * 4 + x * 4 + byte;

    let cell: u8 = rng.gen_range(0..255);
    a.bytes[index as usize] = cell;
    for x in 0..a.size.width {
        for y in 0..a.size.height {
            let index = ((y * a.size.width + x) * 4) as usize;
            a.bytes[index] = rng.gen_range(0..128);
            a.bytes[index + 1] = rng.gen_range(0..128);
            a.bytes[index + 2] = rng.gen_range(0..128);
            a.bytes[index + 3] = rng.gen_range(0..128);
        }
    }
    rng
}
