use std::fmt::{Debug, Display};
use std::ops::{Add, Mul};

// from 0 to MAX
pub trait UnsignedNum:
    Debug + Add<Output = Self> + Mul<Output = Self> + Display + Copy + PartialOrd
{
    fn as_usize(&self) -> usize;
}

#[cfg(test)]
mod tests {
    use crate::UnsignedNum;

    type MyCoolNum = u8;

    impl UnsignedNum for MyCoolNum {
        fn as_usize(&self) -> usize {
            usize::try_from(*self).expect("convert error")
        }
    }

    #[test]
    fn test_unsigned_num_it_works() {
        let c: MyCoolNum = 10 as MyCoolNum;
        let actual = c.as_usize();
        assert_eq!(actual, 10 as usize);
    }
}
