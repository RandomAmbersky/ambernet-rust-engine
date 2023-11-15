mod config;
mod loaders;
mod resources;

use crate::engine::{AsnNodeView2d, AsnTexture, Engine};
use crate::handler::config::{get_config, AsnGameConfig};
use crate::handler::loaders::{load_map, load_tiles};
use crate::handler::resources::{
    MAP_TMX, MAP_TSX, TEXTURE_TIILES_ALPHA_SOURCE, TEXTURE_TIILES_SOURCE,
};
use crate::map::AsnMap;
use crate::tileset::AsnTileSet;
use asn_core::events::{AsnEvent, AsnKeyboardEvent, AsnWindowEvent};
use asn_core::math::{Array2D, Directions, Pos2D, Size2D, UnsignedNum};
use asn_core::traits::{TAsnBaseEngine, TAsnEngine, TAsnHandler};
use asn_core::winapi::scene::{TNodeBase, TNodeQuad, TNodeView2d};
use asn_core::winapi::{AsnTextureFormat, TAsnWinapi, TTexture};
use asn_decoder_image::load_texture;
use rand::prelude::SmallRng;
use rand::{Rng, SeedableRng};
use std::sync::{Arc, Mutex};

const RNG_SEED: u64 = 11;

pub struct Handler {
    config: AsnGameConfig,
    // arc_texture: Arc<Mutex<AsnTexture>>,
    // raw_texture: Array2D<u32, u8>,
    // quad: AsnNodeQuad,
    // quad2: AsnNodeQuad,
    view: AsnNodeView2d,
    player_view: AsnNodeView2d,
    map: AsnMap,
    player_pos: Pos2D<u32>,
    new_player_pos: Pos2D<u32>,
    player_ground: u8,
    look_at: Pos2D<u32>,
    tiles: AsnTileSet,
    rng: SmallRng,
}

impl Handler {
    pub fn new(e: &mut Engine) -> Self {
        let config = get_config();
        let player_pos = Pos2D { x: 1, y: 1 };

        let w = e.get_winapi();
        // let mut quad = AsnNodeQuad::new(w);
        // let mut quad2 = AsnNodeQuad::new(w);

        // let decoded_map = load_tmx(MAP_TMX).unwrap();
        // println!("{:?} ", decoded_map);
        let tiles = load_tiles(MAP_TSX);
        let map = load_map(MAP_TMX);
        // let raw_texture = load_texture(TEXTURE_QUAD_SOURCE);
        // let mut texture = AsnTexture::from_raw(
        //     w,
        //     &raw_texture.bytes,
        //     &raw_texture.size,
        //     AsnTextureFormat::Rgba8,
        // )
        // .unwrap();
        // texture.update_from_raw(w, &raw_texture.bytes).unwrap();
        //
        // quad.set_texture(w, &texture).unwrap();
        // quad2.set_texture(w, &texture).unwrap();

        let raw_tile_texture = load_texture(TEXTURE_TIILES_SOURCE);
        let tile_texture = AsnTexture::from_raw(
            w,
            &raw_tile_texture.bytes,
            &raw_tile_texture.size,
            AsnTextureFormat::Rgba8UnormSrgb,
        )
        .unwrap();

        let raw_tile_aplha_texture = load_texture(TEXTURE_TIILES_ALPHA_SOURCE);
        let tile_alpha_texture = AsnTexture::from_raw(
            w,
            &raw_tile_aplha_texture.bytes,
            &raw_tile_aplha_texture.size,
            AsnTextureFormat::Rgba8UnormSrgb,
        )
        .unwrap();

        let view = AsnNodeView2d::new(w, &tile_texture, &config.view_size, &tiles.get_tile_size());
        let player_view = AsnNodeView2d::new(
            w,
            &tile_alpha_texture,
            &config.view_size,
            &tiles.get_tile_size(),
        );

        let rng = SmallRng::seed_from_u64(RNG_SEED);

        Handler {
            config,
            rng,
            player_pos,
            player_ground: 1,
            new_player_pos: Pos2D::default(),
            look_at: Pos2D::default(),
            // raw_texture,
            // arc_texture: Arc::new(Mutex::new(texture)),
            map, // quad,
            tiles,
            // quad2,
            view,
            player_view,
        }
    }
    fn draw(&mut self, e: &mut Engine) {
        // println!("draw");

        let mut fcx = e.get_winapi().begin_frame().unwrap();
        // self.quad.draw(&mut fcx);
        // self.quad2.draw(&mut fcx);
        self.player_view.draw(&mut fcx);
        self.view.draw(&mut fcx);
        e.get_winapi().end_frame(fcx).unwrap();
        e.get_winapi().send_event(&AsnEvent::UpdateEvent);
    }
    fn update(&mut self, e: &mut Engine) {
        let mut rng = self.rng.clone();

        if self.new_player_pos != self.player_pos {
            self.map.set_cell(0, &self.player_pos, self.player_ground);

            self.player_pos = self.new_player_pos;

            self.player_ground = self.map.get_cell(0, &self.player_pos);

            self.map
                .set_cell(0, &self.player_pos, self.config.player_cell);

            self.look_at = self
                .map
                .get_size_2d()
                .look_at_window(&self.player_pos, &self.view.get_size());
            println!("player pos: {:?}", self.player_pos);
            println!("look_at pos: {:?}", self.look_at);
            fill_view(&self.map, &self.look_at, &mut self.view);
            fill_view(&self.map, &self.look_at, &mut self.player_view);
        }

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

        // for _ in 0..10 {
        //     rng = randomize_view_cell(rng, &mut self.view);
        // }

        self.view.update(e.get_winapi());
        // self.player_view.update(e.get_winapi());
        self.rng = rng;
    }
}

impl Handler {
    fn handle_move_player(&mut self, dir: Directions) {
        let result = move_player(&self.new_player_pos, &self.map.get_size_2d(), dir);
        match result {
            None => {}
            Some(t) => self.new_player_pos = t,
        };
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
                    self.player_view.set_screen_size(size);
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
            AsnEvent::KeyboardEvent(e) => {
                println!("KeyboardEvent {:?}", &e);
                match e {
                    AsnKeyboardEvent::Released(_) => {}
                    AsnKeyboardEvent::Pressed(scancode) => match scancode {
                        124 => self.handle_move_player(Directions::Right),
                        123 => self.handle_move_player(Directions::Left),
                        125 => self.handle_move_player(Directions::Up),
                        126 => self.handle_move_player(Directions::Down),
                        _ => {}
                    },
                };
                println!("AsnEvent::KeyboardEvent {:?}", e);
            }
            _ => {}
        }
    }
}

// fn randomize_view_cell(mut rng: SmallRng, v: &mut AsnNodeView2d) -> SmallRng {
//     let x: u32 = rng.gen_range(0..MAP_VIEW_SIZE.width);
//     let y: u32 = rng.gen_range(0..MAP_VIEW_SIZE.height);
//     let c: u8 = rng.gen_range(0..128);
//     v.set_cell(&Pos2D { x, y }, c).unwrap();
//     rng
// }

// fn fill_by_index(v: &mut AsnNodeView2d) {
//     let mut index: u8 = 0;
//     for y in 0..MAP_VIEW_SIZE.height {
//         for x in 0..MAP_VIEW_SIZE.width {
//             v.set_cell(&Pos2D { x, y }, index).unwrap();
//             index = index.wrapping_add(1);
//         }
//     }
// }

// fn randomize_view(mut rng: SmallRng, v: &mut AsnNodeView2d) -> SmallRng {
//     for x in 0..MAP_VIEW_SIZE.width {
//         for y in 0..MAP_VIEW_SIZE.height {
//             let c: u8 = rng.gen_range(0..128);
//             v.set_cell(&Pos2D { x, y }, c).unwrap();
//         }
//     }
//     rng
// }

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

fn fill_view(map: &AsnMap, start_pos: &Pos2D<u32>, view: &mut AsnNodeView2d) {
    for y in 0..view.get_size().height {
        for x in 0..view.get_size().width {
            let cell_pos = Pos2D {
                x: start_pos.x + x,
                y: start_pos.y + y,
            };
            let cell = map.get_cell(0, &cell_pos) - 1;
            view.set_cell(&Pos2D { x, y }, cell).unwrap();
        }
    }
}

fn move_player(
    start_pos: &Pos2D<u32>,
    map_size: &Size2D<u32>,
    dir: Directions,
) -> Option<Pos2D<u32>> {
    let mut pos = *start_pos;
    match dir {
        Directions::Up => match pos.y.checked_add(1) {
            None => return None,
            Some(t) => {
                pos.y = t;
            }
        },
        Directions::Down => match pos.y.checked_sub(1) {
            None => return None,
            Some(t) => {
                pos.y = t;
            }
        },
        Directions::Left => match pos.x.checked_sub(1) {
            None => return None,
            Some(t) => {
                pos.x = t;
            }
        },
        Directions::Right => match pos.x.checked_add(1) {
            None => return None,
            Some(t) => {
                pos.x = t;
            }
        },
    }

    if map_size.is_pos_into(&pos) {
        return Some(pos);
    }
    None
}
