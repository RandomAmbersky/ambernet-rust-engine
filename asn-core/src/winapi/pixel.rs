use crate::math::UnsignedNum;

#[allow(dead_code)]
pub struct PixelRGBA<E: UnsignedNum> {
    r: E,
    g: E,
    b: E,
    a: E,
}

#[cfg(test)]
mod tests {
    use crate::engine::core::winapi::PixelRGBA;
    use crate::*;

    #[test]
    fn it_works() {
        let myPixel = PixelRGBA {
            r: 10_u8,
            g: 10_u8,
            b: 10_u8,
            a: 10_u8,
        };

        assert_eq!(myPixel.r, 10_u8);
    }
}
