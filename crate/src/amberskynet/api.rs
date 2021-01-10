pub trait LoggerApi {
    fn log(&self, mess: &str);
}

pub trait AmberNetApi<LoggerType> {
    fn new() -> Self;
    fn get_log(&self) -> &LoggerType;
}

