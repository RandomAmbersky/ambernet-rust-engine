use std::fmt::{Debug, Display};
use std::ops::{Add, Mul, Shr, Sub};

// from 0 to MAX
pub trait UnsignedNum:
    Debug
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Shr<Output = Self>
    + Display
    + Copy
    + PartialOrd
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
    use crate::math::UnsignedNum;

    type MyType = u8;

    struct MyStruct<T: UnsignedNum> {
        pub s: T,
    }

    impl<T> MyStruct<T>
    where
        T: UnsignedNum,
    {
        pub fn add(&mut self, delta: T) {
            self.s = self.s + delta;
        }
    }

    #[test]
    fn test_unsigned_num_it_works() {
        let mut c = MyStruct { s: 10 as MyType };
        assert_eq!(c.s, 10);
        c.add(5);
        assert_eq!(c.s, 15);
    }
}
