pub trait WinApiTrait {
    fn new(x: u32, y: u32) -> Self;
    fn resize(&mut self, x: u32, y: u32);
}
