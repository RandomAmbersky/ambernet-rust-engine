mod position;
mod actor;
mod player;
pub mod defines;
mod render;
mod background;

use specs::{World, WorldExt, Builder, Join, Entity, Component, VecStorage};
use asn_core::{Array2D, Point2D};
use position::Position;
use actor::Actor;
use amberskynet_logger_web::LoggerWeb;
use asn_view_2d::View2D;
use defines::{Action, Direction};
use player::Player;
use crate::logic::background::Background;
use crate::logic::defines::{Key, PLAYER_SPRITE_ID};

#[derive(Default, Debug)]
pub struct Logic {
    is_need_view_update: bool
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

impl Logic {
    pub fn set_map(&mut self, w: &mut World, map: Array2D) -> Result<(), String> {
        let mut my_map = Map {
            map
        };

        let player_pos = Point2D { x: 1, y: 1 };

        let background_cell = my_map.map.get_cell(&player_pos)?;
        my_map.map.set_cell(&player_pos, PLAYER_SPRITE_ID)?;

        w.create_entity()
            .with(Player{})
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

        let mut dir = Direction::from_key(&key)?;

        // inverse map
        match dir {
            Direction::Up => {
                dir = Direction::Down
            },
            Direction::Down => {
                dir = Direction::Up
            }
            _ => {}
        }

        self.is_need_view_update = true;
        move_player(w, &dir)?;
        Ok(())
    }

    pub fn update(&mut self, w: &mut World, view: &mut View2D, _time: f32) -> Result<(), String> {
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

fn move_player(w: &mut World, dir: &Direction) -> Result<(), String> {
    let players = w.read_storage::<Player>();
    let mut positions = w.write_storage::<Position>();
    let mut backgrounds = w.write_storage::<Background>();

    let mut my_map = w.fetch_mut::<Map>();


    for (_player, position, background) in (&players, &mut positions, &mut backgrounds).join() {
        let new_pos = match move_point(&position.pos, &my_map.map, dir) {
            Ok(t) => t,
            Err(_) => continue
        };
        let background_cell = my_map.map.get_cell(&new_pos)?;
        my_map.map.set_cell(&position.pos, background.cell)?;
        my_map.map.set_cell(&new_pos, PLAYER_SPRITE_ID)?;
        position.pos = new_pos;
        background.cell = background_cell;
    }
    Ok(())
}

fn move_point(pos: &Point2D, map: &Array2D, dir: &Direction) -> Result<Point2D, String> {
    let dir_delta = dir.as_delta();
    let new_pos = pos.add(&dir_delta)?;
    if map.is_valid_pos(&new_pos) {
        return Ok(new_pos)
    }
    Err(String::from("can't move "))
}
