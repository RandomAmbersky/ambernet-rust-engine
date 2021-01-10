use super::super::api;

pub struct Render {}

impl api::RenderApi for Render {
    fn new() -> Self {
        Self {}
    }
    fn draw(&self) {}
}
