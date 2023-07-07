use crate::Size2D;

#[derive(Debug)]
pub struct AsnWindowId(pub usize);

#[derive(Debug)]
pub enum AsnWindowEvent {
    Resized(Size2D<u32>),
    RedrawRequested,
}

#[derive(Debug)]
pub enum AsnEvent {
    Empty,
    WindowEvent {
        window_id: AsnWindowId,
        event: AsnWindowEvent,
    },
}
