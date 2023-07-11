use crate::Size2D;

pub trait AsnWinapiTrait {
    fn window_resize(&mut self, size: &Size2D<u32>);
}
