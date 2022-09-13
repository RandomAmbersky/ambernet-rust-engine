mod position;

use specs::{World, WorldExt, Builder};
use position::Position;

pub struct Logic {
    world: World
}

pub fn new () -> Logic {
    let mut world = World::new();
    world.register::<Position>();
    Logic {
        world
    }
}

impl Logic {

}
