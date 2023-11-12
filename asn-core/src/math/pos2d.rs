use super::UnsignedNum;
use crate::math::direction::Directions;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Pos2D<T: UnsignedNum> {
    pub x: T,
    pub y: T,
}

#[cfg(test)]
mod tests {
    use crate::math::Pos2D;

    #[test]
    fn check_pos_2d() {
        let value: u32 = 10;
        let a = Pos2D { x: value, y: value };
        assert_eq!(a.x, value);
        assert_eq!(a.y, value);
    }
}
