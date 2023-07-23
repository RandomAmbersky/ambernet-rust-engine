#[derive(Debug)]
pub enum AsnRenderError {
    CustomError(String),
}

#[derive(Debug)]
pub enum AsnError {
    Empty,
    RenderError(AsnRenderError),
}
