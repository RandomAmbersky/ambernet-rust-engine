use super::UnsignedNum;

#[derive(Default, Debug)]
pub struct Pos3D<T: UnsignedNum> {
    pub x: T,
    pub y: T,
    pub z: T,
}

#[cfg(test)]
mod tests {
    use crate::math::pos3d::Pos3D;

    #[test]
    fn check_pos_2d() {
        let value: u32 = 10;
        let a = Pos3D {
            x: value,
            y: value,
            z: value,
        };
        assert_eq!(a.x, value);
        assert_eq!(a.y, value);
        assert_eq!(a.z, value);
    }
}
