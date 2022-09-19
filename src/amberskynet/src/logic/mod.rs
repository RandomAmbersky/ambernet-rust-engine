pub mod defines;
mod components;
mod entities;

use specs::{Builder, Join, World, WorldExt};
use asn_core::{Array2D, Direction, Point2D};
use components::position::Position;
use components::actor::Actor;
use amberskynet_logger_web::LoggerWeb;
use asn_view_2d::View2D;
use components::player::Player;
use crate::Action;
use components::background::Background;
use components::player;
use crate::logic::defines::{Key, PLAYER_SPRITE_ID};

use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct Logic {
    is_need_view_update: bool,
    tile_list: HashMap<u32, String>
}

#[derive(Default, Debug)]
struct Map {
    map: Array2D
}

pub fn create_world () -> World {
    let mut world = World::new();
    world.register::<Position>();
    world.register::<Player>();
    world.register::<Background>();
    world.register::<Actor>();

    world
}

fn move_player(w: &mut World, dir: &Direction) -> Result<(), String> {
    let mut players = w.write_storage::<Player>();
    let mut positions = w.write_storage::<Position>();
    let mut backgrounds = w.write_storage::<Background>();

    let mut my_map = w.fetch_mut::<Map>();

    for (player, position, background) in (&mut players, &mut positions, &mut backgrounds).join() {
        if &player.dir != dir {
            player.dir = *dir;
            let new_sprite_num = player::dir_to_sprite(dir);
            my_map.map.set_cell(&position.pos, new_sprite_num)?;
            continue
        }
        let new_pos = match my_map.map.move_point(&position.pos, dir) {
            Ok(t) => t,
            Err(_) => continue
        };
        let background_cell = my_map.map.get_cell(&new_pos)?;
        my_map.map.set_cell(&position.pos, background.cell)?;
        let new_sprite_num = player::dir_to_sprite(dir);
        my_map.map.set_cell(&new_pos, new_sprite_num)?;
        position.pos = new_pos;
        background.cell = background_cell;
    }
    Ok(())
}

impl Logic {
    pub fn set_tile_type(&mut self, id: u32, name: &String) -> Result<(), String> {
        // let mess = format!("tile is: {:?} {:?}", id, name);
        // LoggerWeb::log(&mess);
        self.tile_list.insert(id, String::from(name));
        Ok(())
    }

    pub fn get_tile_type(&self, id: &u32) -> Result<(), String> {
        let tile_type = self.tile_list.get(id).ok_or(format!("Item id {} not found", id));
        let mess = format!("tile is: {:?}", tile_type);
        LoggerWeb::log(&mess);
        Ok(())
    }

    pub fn set_background(&mut self, w: &mut World, map: Array2D) -> Result<(), String> {
        Ok(())
    }

    pub fn set_map(&mut self, w: &mut World, map: Array2D) -> Result<(), String> {
        for i in 0..256 {
            match self.get_tile_type(&i) {
                Ok(t) => {
                    LoggerWeb::log(format!("{:?}", t).as_str());
                }
                Err(err) => {
                        LoggerWeb::log(&err);
                }
            }
        }

        let mut my_map = Map {
            map
        };

        let player_pos = Point2D { x: 1, y: 1 };

        let background_cell = my_map.map.get_cell(&player_pos)?;
        my_map.map.set_cell(&player_pos, PLAYER_SPRITE_ID)?;

        w.create_entity()
            .with(Player::default())
            .with(Actor{})
            .with(Background {
                cell: background_cell
            })
            .with(Position{
                pos: player_pos
            })
            .build();

        w.insert(my_map);
        self.is_need_view_update = true;
        Ok(())
    }

    pub fn process_key(&mut self, w: &mut World, key: Key) -> Result<(), String> {
        let mess = format!("process_key {:?}", key);
        LoggerWeb::log(&mess);

        match key {
            Key::None => {},
            Key::Up => {
                process_moving_dir(w, &Direction::Down)?;
            },
            Key::Down => {
                process_moving_dir(w, &Direction::Up)?;
            },
            Key::Left => {
                process_moving_dir(w, &Direction::Left)?;
            },
            Key::Right => {
                process_moving_dir(w, &Direction::Right)?;
            },
            Key::Fire => {
                process_action(w, &Action::Use)?;
            }
        }

        self.is_need_view_update = true;
        Ok(())
    }

    pub fn update(&mut self, w: &mut World, view: &mut View2D, _time: f32) -> Result<(), String> {
        w.maintain();
        if self.is_need_view_update {
            self.update_view(w, view)?;
            self.is_need_view_update = false;
        }
        Ok(())
    }

    fn update_view(&self, w: &World, view: &mut View2D) -> Result<(), String> {
        let players = w.read_storage::<Player>();
        let positions = w.read_storage::<Position>();
        let my_map = w.fetch::<Map>();
        for (_player, position) in (&players, &positions).join() {
            view.look_at(&position.pos, &my_map.map)?;
        }
        Ok(())
    }
}

pub fn process_moving_dir (w: &mut World, dir: &Direction) -> Result<(), String> {
    move_player(w, dir)?;
    Ok(())
}

pub fn process_action (_w: &mut World, _act: &Action) -> Result<(), String> {
    Ok(())
}
