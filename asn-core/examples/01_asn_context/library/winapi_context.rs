use crate::library::winapi_trait::WinApiTrait;

pub struct WinapiContext {
    x: u32,
    y: u32,
}

impl WinApiTrait for WinapiContext {
    fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
    fn resize(&mut self, x: u32, y: u32) {
        self.x = x;
        self.y = y;
        println!("{:} {:}", self.x, self.y);
    }
}
