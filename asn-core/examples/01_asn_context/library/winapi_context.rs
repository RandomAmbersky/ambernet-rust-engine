use asn_core::{AsnError, AsnWinapiTrait, Size2D};

pub struct WinapiContext {
    x: u32,
    y: u32,
}

impl WinapiContext {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}

impl AsnWinapiTrait for WinapiContext {
    fn window_resize(&mut self, size: &Size2D<u32>) {
        self.x = size.width;
        self.y = size.height;
        println!("{:} {:}", self.x, self.y);
    }

    fn redraw(&mut self) -> Option<AsnError> {
        todo!()
    }
}
