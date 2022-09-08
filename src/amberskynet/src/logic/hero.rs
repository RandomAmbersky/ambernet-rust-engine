use asn_core::Point2D;
use crate::logic::defines::{Action, Directions};

#[derive(Default, Debug)]
struct Hero {
    pos: Point2D
}

impl Hero {
    pub fn do_some(&self, dir: Directions, act: Action) -> Result<(), String> {
        Ok(())
    }
}
