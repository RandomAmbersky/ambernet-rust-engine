mod position;
mod actor;
mod player;
mod defines;

use specs::{World, WorldExt, Builder, Join, Entity};
use asn_core::Point2D;
use position::Position;
use actor::Actor;
use defines::{Action, Direction};
use player::Player;

pub struct Logic {
    world: World
}

pub fn new () -> Logic {
    let mut world = World::new();
    world.register::<Position>();
    world.register::<Player>();

    world.create_entity().with(Player{}).build();
    Logic {
        world
    }
}

impl Logic {
    pub fn do_action(&mut self, act: Action, dir: Direction) {
        let delta = dir.to_delta();
        
        let mut positions = self.world.write_storage::<Position>();
        let mut players = self.world.write_storage::<Player>();

        for (_player, _pos) in (&mut players, &mut positions).join() {

        }
    }

    pub fn do_action_at(&mut self, act: Action, pos: &Point2D) {
        return match act {
            Action::Move => self.do_move(pos),
            Action::Use => self.do_use(pos)
        }
    }

    pub fn do_move(&mut self, pos: &Point2D) {

    }

    pub fn do_use(&mut self, pos: &Point2D) {

    }
}
