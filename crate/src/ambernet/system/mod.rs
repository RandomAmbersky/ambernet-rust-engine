mod renderer;
mod sound;

use renderer::Renderer;
use sound::Sound;

pub fn get_renderer() -> Renderer {
    Renderer::new()
}
pub fn get_sound() -> Sound {
    Sound::new()
}
