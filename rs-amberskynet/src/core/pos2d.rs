pub struct Pos2D<S> {
    pub x: S,
    pub y: S,
}

#[cfg(test)]
mod tests {
    use crate::core::pos2d::Pos2D;

    #[test]
    fn it_works() {
        let value: u32 = 10;
        let pos = Pos2D { x: value, y: value };
        assert_eq!(pos.x, value);
        assert_eq!(pos.y, value);
    }

    #[test]
    fn it_eq() {
        let value: u32 = 10;
        let pos = Pos2D { x: value, y: value };

        let pos2 = pos;
        assert_eq!(pos2.x, value);
        assert_eq!(pos2.y, value);
    }
}
