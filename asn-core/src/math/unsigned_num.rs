use std::fmt::{Debug, Display};
use std::ops::{Add, Div, Mul, Shr, Sub};

// from 0 to MAX
pub trait UnsignedNum:
    Debug
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Shr<Output = Self>
    + Display
    + Copy
    + Sized
    + PartialOrd
{
    const ZERO: Self;
    const ONE: Self;
    fn as_usize(&self) -> usize;
    fn from_usize(c: usize) -> Self;
}

// macro_rules! empty_trait_impl {
//     ($name:ident for $($t:ty)*) => ($(
//         impl $name for $t {
//             const ZERO: Self = 0;
//             const ONE: Self = 1;
//             fn as_usize(&self) -> usize {
//                 usize::try_from(*self).expect("convert error")
//             }
//         }
//     )*)
// }

// empty_trait_impl!(UnsignedNum for u8 u16 u32 u64);

impl UnsignedNum for u32 {
    const ZERO: Self = 0;
    const ONE: Self = 1;

    fn as_usize(&self) -> usize {
        usize::try_from(*self).expect("convert error")
    }

    fn from_usize(c: usize) -> Self {
        usize::try_into(c).expect("convert error")
    }
}

impl UnsignedNum for u64 {
    const ZERO: Self = 0;
    const ONE: Self = 1;

    fn as_usize(&self) -> usize {
        usize::try_from(*self).expect("convert error")
    }

    fn from_usize(c: usize) -> Self {
        usize::try_into(c).expect("convert error")
    }
}

#[cfg(test)]
mod tests {
    use crate::math::UnsignedNum;

    type MyType = u8;

    struct MyStruct<T: UnsignedNum> {
        pub s: T,
    }

    // impl<T> MyStruct<T>
    // where
    //     T: UnsignedNum,
    // {
    //     pub fn add(&mut self, delta: T) {
    //         self.s = self.s + delta;
    //     }
    //     pub fn inc(&mut self) {
    //         self.s = self.s + 1_u16.into();
    //     }
    // }

    // #[test]
    // fn test_unsigned_num_it_works() {
    //     let mut c = MyStruct { s: 10 as MyType };
    //     assert_eq!(c.s, 10);
    //     c.add(5);
    //     assert_eq!(c.s, 15);
    // }
}
