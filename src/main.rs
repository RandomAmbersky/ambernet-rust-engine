extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use sdl2::EventPump;

struct MainData {
    event_pump: EventPump,
    canvas: WindowCanvas,
}

fn init() -> Result<MainData, String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("rust-sdl2 demo: Video", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    canvas.set_draw_color(Color::RGB(255, 0, 0));
    canvas.clear();
    canvas.present();
    let event_pump = sdl_context.event_pump()?;
    let data = MainData { event_pump, canvas };
    Ok(data)
}

pub fn main() -> Result<(), String> {
    let mut d = init()?;

    'running: loop {
        for event in d.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        d.canvas.clear();
        d.canvas.present();
    }

    Ok(())
}
