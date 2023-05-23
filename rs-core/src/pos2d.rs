use crate::UnsignedNum;

pub struct Pos2D<T: UnsignedNum> {
    pub x: T,
    pub y: T,
}

pub struct MyStruct {
    x: u32,
    y: u32,
}

impl Default for MyStruct {
    fn default() -> Self {
        MyStruct { x: 10, y: 10 }
    }
}

#[cfg(test)]
mod tests {
    use crate::pos2d::{MyStruct, Pos2D};

    #[test]
    fn check_pos_2d() {
        let value: u32 = 10;
        let a = Pos2D { x: value, y: value };
        assert_eq!(a.x, value);
        assert_eq!(a.y, value);

        let s = MyStruct::default();
    }
}
