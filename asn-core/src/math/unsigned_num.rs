use std::fmt::{Debug, Display};
use std::ops::{Add, Mul};

// from 0 to MAX
pub trait UnsignedNum:
    Debug + Add<Output = Self> + Mul<Output = Self> + Display + Copy + PartialOrd
{
    fn as_usize(&self) -> usize;
}

macro_rules! empty_trait_impl {
    ($name:ident for $($t:ty)*) => ($(
        impl $name for $t {
            fn as_usize(&self) -> usize {
                usize::try_from(*self).expect("convert error")
            }
        }
    )*)
}

empty_trait_impl!(UnsignedNum for u8 u16 u32 u64);

#[cfg(test)]
mod tests {
    type MyCoolNum = u8;

    #[test]
    fn test_unsigned_num_it_works() {
        let c: MyCoolNum = 10 as MyCoolNum;
        let actual = c.as_usize();
        assert_eq!(actual, 10_usize);
    }
}
