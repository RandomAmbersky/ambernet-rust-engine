pub trait LoggerApi {
    fn log(&self, mess: &str);
}

pub trait RenderApi {
    fn new() -> Self;
    fn resize(&self, _width: f32, _height: f32);
    fn draw(&self);
}

pub trait AmberNetApi<LoggerType, RenderType> {
    fn new() -> Self;
    fn update(&self, _time: f32);
    fn get_log(&self) -> &LoggerType;
    fn get_render(&self) -> &RenderType;
}
