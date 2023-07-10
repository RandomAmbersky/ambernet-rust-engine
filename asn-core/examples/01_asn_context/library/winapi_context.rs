pub struct WinapiContext {
    x: u32,
    y: u32,
}

impl WinapiContext {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
    pub fn resize(&mut self, x: u32, y: u32) {
        self.x = x;
        self.y = y;
    }
}
