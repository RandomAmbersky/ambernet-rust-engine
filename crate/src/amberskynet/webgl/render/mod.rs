use super::super::api;

pub struct Render {}

impl api::RenderApi for Render {
    fn new() -> Self {
        Self {}
    }

    fn resize(&self, _width: f32, _height: f32) {

    }

    fn draw(&self) {

    }
}
