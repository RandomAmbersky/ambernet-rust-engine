use crate::engine::core::math::Size2D;

#[derive(Debug)]
pub enum AsnWindowEvent {
    Resized(Size2D<u32>),
    RedrawRequested,
    CloseRequested,
}

#[derive(Debug)]
pub enum AsnEvent {
    Empty,
    WindowEvent(AsnWindowEvent),
}
