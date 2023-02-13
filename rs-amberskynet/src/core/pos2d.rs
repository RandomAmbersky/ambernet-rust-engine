use std::fmt::Display;

#[derive(Default, Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct Pos2D<S>
where
    S: Display,
{
    pub x: S,
    pub y: S,
}
