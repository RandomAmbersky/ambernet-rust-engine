use rs_core::UnsignedNum;

#[allow(dead_code)]
pub struct PixelRGBA<E> {
    r: E,
    g: E,
    b: E,
    a: E,
}

#[cfg(test)]
mod tests {
    use crate::*;
    use rs_core::UnsignedNum;

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
