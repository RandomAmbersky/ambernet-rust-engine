mod config;
mod info_loader;
mod loaders;
mod resources;

use crate::engine::{AsnNodeView2d, AsnTexture, Engine};
use crate::handler::config::{get_config, AsnGameConfig};
use crate::handler::info_loader::load_info;
use crate::handler::loaders::{load_map, load_tiles};
use crate::handler::resources::{
    MAP_TMX, MAP_TSX, TEXTURE_TIILES_ALPHA_SOURCE, TEXTURE_TIILES_SOURCE,
};
use crate::map::AsnMap;
use crate::tileset::AsnTileSet;
use asn_core::events::{AsnEvent, AsnKeyboardEvent, AsnWindowEvent};
use asn_core::keys::JoystickKeys::{KeyDown, KeyLeft, KeyRight, KeyUp};
use asn_core::keys::{JoystickKeysSet, KeysSetOperations};
use asn_core::math::{Directions, Fps, Pos2D, Size2D};
use asn_core::traits::{TAsnBaseEngine, TAsnEngine, TAsnHandler};
use asn_core::winapi::scene::{TNodeBase, TNodeQuad, TNodeView2d};
use asn_core::winapi::{AsnTextureFormat, TAsnWinapi, TTexture};
use asn_decoder_image::load_texture;
use asn_logger::info;
use rand::prelude::SmallRng;
use rand::{Rng, SeedableRng};

const RNG_SEED: u64 = 11;
const UPDATE_THROTTLE: f32 = 1.0 / 30.0;

pub struct Handler {
    config: AsnGameConfig,
    keys: JoystickKeysSet,
    update_fps: Fps,
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
        load_info();

        let config = get_config();
        let player_pos = Pos2D { x: 1, y: 1 };

        let w = e.get_winapi();
        let tiles = load_tiles(MAP_TSX);
        let map = load_map(MAP_TMX);

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
            update_fps: Fps::new(UPDATE_THROTTLE),
            config,
            rng,
            player_pos,
            player_ground: 1,
            new_player_pos: Pos2D::default(),
            look_at: Pos2D::default(),
            map,
            tiles,
            view,
            player_view,
            keys: JoystickKeysSet::default(),
        }
    }
    fn draw(&mut self, e: &mut Engine) {
        let mut fcx = e.get_winapi().begin_frame().unwrap();
        self.player_view.draw(&mut fcx);
        self.view.draw(&mut fcx);
        e.get_winapi().end_frame(fcx).unwrap();
        e.get_winapi().send_event(&AsnEvent::UpdateEvent);
    }

    fn update(&mut self, e: &mut Engine) {
        let mut rng = self.rng.clone();

        // info!(
        //     "delta: {:?} {:?}",
        //     self.update_fps.get_delta().as_secs_f32(),
        //     UPDATE_THROTTLE
        // );

        if self.update_fps.tick() {
            self.handle_keyset();
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
                // fill_view(&self.map, &self.look_at, &mut self.player_view);
                fill_view(&self.map, &self.look_at, &mut self.view);
            }
        }

        // rng = fill_random_view(rng, &mut self.view);
        self.view.update(e.get_winapi());
        self.rng = rng;
    }
}

impl Handler {
    fn handle_keyset(&mut self) {
        if self.keys.is_set(KeyUp as u8) {
            self.handle_move_player(Directions::Up)
        }
        if self.keys.is_set(KeyDown as u8) {
            self.handle_move_player(Directions::Down)
        }
        if self.keys.is_set(KeyLeft as u8) {
            self.handle_move_player(Directions::Left)
        }
        if self.keys.is_set(KeyRight as u8) {
            self.handle_move_player(Directions::Right)
        }
    }
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
        // info!("handle {:?} event", &evt);
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
                info!("handle {:?} event", &evt);
                println!("KeyboardEvent {:?}", &e);
                match e {
                    AsnKeyboardEvent::Released(scancode) => match scancode {
                        39 => self.keys.reset(KeyRight as u8),
                        124 => self.keys.reset(KeyRight as u8),
                        37 => self.keys.reset(KeyLeft as u8),
                        123 => self.keys.reset(KeyLeft as u8),
                        38 => self.keys.reset(KeyDown as u8),
                        126 => self.keys.reset(KeyDown as u8),
                        40 => self.keys.reset(KeyUp as u8),
                        125 => self.keys.reset(KeyUp as u8),
                        _ => {
                            info!("handle {:?} event", &evt);
                        }
                    },

                    AsnKeyboardEvent::Pressed(scancode) => match scancode {
                        39 => self.keys.set(KeyRight as u8),
                        124 => self.keys.set(KeyRight as u8),
                        37 => self.keys.set(KeyLeft as u8),
                        123 => self.keys.set(KeyLeft as u8),
                        38 => self.keys.set(KeyDown as u8),
                        126 => self.keys.set(KeyDown as u8),
                        40 => self.keys.set(KeyUp as u8),
                        125 => self.keys.set(KeyUp as u8),
                        _ => {}
                    },
                };
                println!("AsnEvent::KeyboardEvent {:?}", e);
            }
            _ => {}
        }
    }
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

fn fill_random_view(mut rng: SmallRng, view: &mut AsnNodeView2d) -> SmallRng {
    for y in 0..view.get_size().height {
        for x in 0..view.get_size().width {
            let cell = rng.gen_range(0..255);
            view.set_cell(&Pos2D { x, y }, cell).unwrap();
        }
    }
    rng
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
