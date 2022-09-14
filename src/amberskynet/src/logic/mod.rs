mod position;
mod actor;
mod player;
pub mod defines;

use specs::{World, WorldExt, Builder, Join, Entity};
use asn_core::Point2D;
use position::Position;
use actor::Actor;
use amberskynet_logger_web::LoggerWeb;
use defines::{Action, Direction};
use player::Player;

pub struct Logic {
    world: World
}

pub fn new () -> Logic {
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
    Logic {
        world
    }
}

fn move_player (w: &mut World, dir: &Direction) {
    let positions = w.write_storage::<Position>();
    let players = w.read_storage::<Player>();

    let delta = dir.as_delta();

    for (_player, position) in (&players, &positions).join() {
        position.pos;
    }
}

impl Logic {
    pub fn do_action(&mut self, act: Action, dir: Direction) {
        let mess = format!("do_action");
        LoggerWeb::log(&mess);

        match act {
                Action::Move => {
                    move_player(&mut self.world, &dir);
                },
                Action::Use => {
                    let mess = format!("do_action {:?}", act);
                    LoggerWeb::log(&mess);
                }
            };
    }

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
