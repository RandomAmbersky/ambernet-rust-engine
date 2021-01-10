pub trait LoggerApi {
    fn log(&self, mess: &str);
}

pub trait RenderApi {
    fn new() -> Self;
    fn draw(&self);
}

pub trait AmberNetApi<LoggerType, RenderType> {
    fn new() -> Self;
    fn get_log(&self) -> &LoggerType;
    fn get_render(&self) -> &RenderType;
}
