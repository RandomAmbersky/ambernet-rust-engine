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
    use crate::winapi::PixelRGBA;


    #[test]
    fn it_works() {
        let my_pixel = PixelRGBA {
            r: 10_u8,
            g: 10_u8,
            b: 10_u8,
            a: 10_u8,
        };

        assert_eq!(my_pixel.r, 10_u8);
    }
}
