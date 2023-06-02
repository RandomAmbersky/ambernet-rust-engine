// not use UnsignedNum because CellType may be uuid for example

pub trait CellType: Copy {
    const ZERO: Self;
}
