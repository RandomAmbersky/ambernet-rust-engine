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

#[derive(Default, Debug)]
pub struct Logic {
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

pub fn set_map(w: &mut World, map: Array2D) {
    let my_map = Map {
        map
    };
    w.insert(my_map);
}

impl Logic {
    pub fn update_view(&self, w: &World, view: &mut View2D) -> Result<(), String> {
        let my_map = w.fetch::<Map>();

        let mess = format!("update_view: {:?}", my_map.map.size);
        LoggerWeb::log(&mess);

        view.look_at(&self.pos, &my_map.map)?;
        Ok(())
    }
}

impl Logic {
    pub fn do_action_at(&mut self, act: Action, pos: Point2D) {
        return match act {
            Action::Move => self.do_move(pos),
            Action::Use => self.do_use(pos)
        }
    }

    pub fn do_move(&mut self, pos: Point2D) {
    }

    pub fn do_use(&mut self, pos: Point2D) {
    }
}
