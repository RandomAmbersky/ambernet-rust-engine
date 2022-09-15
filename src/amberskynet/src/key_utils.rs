use amberskynet_logger_web::LoggerWeb;
use crate::logic::defines::Key;

pub fn match_key (evt: &web_sys::KeyboardEvent ) -> Key {
    let key = evt.key();
    let mess = format!("KeyboardEvent: {:?}", &key);
    LoggerWeb::log(&mess);

    let matched_key = match key.as_ref() {
        "ArrowDown" => Key::Down,
        "ArrowUp" => Key::Up,
        "ArrowLeft" => Key::Left,
        "ArrowRight" => Key::Right,
        _ => Key::None,
    };
    matched_key
}
