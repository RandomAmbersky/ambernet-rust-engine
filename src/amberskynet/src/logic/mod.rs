mod position;
mod actor;
mod player;
pub mod defines;
mod render;

use specs::{World, WorldExt, Builder, Join, Entity};
use asn_core::{Array2D, Point2D};
use position::Position;
use actor::Actor;
use amberskynet_logger_web::LoggerWeb;
use asn_view_2d::View2D;
use defines::{Action, Direction};
use player::Player;
use crate::logic::defines::Key;

#[derive(Default, Debug)]
pub struct Logic {
    is_need_view_update: bool,
    pos: Point2D
}

#[derive(Default, Debug)]
struct Map {
    map: Array2D
}

pub fn create_world () -> World {
    let mut world = World::new();
    world.register::<Position>();
    world.register::<Player>();
    world.register::<Actor>();

    world.create_entity()
        .with(Player{})
        .with(Actor{})
        .with(Position{
            pos: Point2D{ x: 10, y: 10 }
        })
        .build();
    world
}

impl Logic {
    pub fn set_map(&mut self, w: &mut World, map: Array2D) {
        let my_map = Map {
            map
        };
        w.insert(my_map);
        self.is_need_view_update = true;
    }

    pub fn process_key(&mut self, w: &mut World, key: Key) {
        let mess = format!("process_key {:?}", key);
        LoggerWeb::log(&mess);
    }

    pub fn update(&mut self, w: &mut World, view: &mut View2D, _time: f32) -> Result<(), String> {
        if self.is_need_view_update {
            self.update_view(w, view)?;
            self.is_need_view_update = false;
        }
        Ok(())
    }
    fn update_view(&self, w: &World, view: &mut View2D) -> Result<(), String> {
        let my_map = w.fetch::<Map>();
        view.look_at(&self.pos, &my_map.map)?;
        Ok(())
    }
}
