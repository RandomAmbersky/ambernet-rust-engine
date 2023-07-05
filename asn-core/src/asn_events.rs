use crate::Size2D;

#[derive(Debug)]
pub struct WindowId(pub usize);

#[derive(Debug)]
pub enum WindowEvent {
    Resized(Size2D<u32>),
    RedrawRequested,
}

#[derive(Debug)]
pub enum AsnEvent {
    Empty,
    WindowEvent {
        window_id: WindowId,
        event: WindowEvent,
    },
}
