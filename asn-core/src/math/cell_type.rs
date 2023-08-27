// not use UnsignedNum because CellType may be uuid for example

pub trait CellType: Copy {
    const ZERO: Self;
}

impl CellType for u8 {
    const ZERO: Self = 0_u8;
}
