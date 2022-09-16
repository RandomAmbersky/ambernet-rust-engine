use crate::Delta2D;

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
pub enum Direction {
    #[default]
    Down,
    // DownLeft,
    Left,
    // LeftUp,
    Up,
    // UpRight,
    Right
}

impl Direction {
    pub fn as_delta(&self) -> Delta2D {
        let mut res = Delta2D::default();
        match self {
            Direction::Down => res.y -=1,
            Direction::Up => res.y +=1,
            Direction::Left => res.x -=1,
            Direction::Right => res.x +=1,
        }
        return res;
    }
}
