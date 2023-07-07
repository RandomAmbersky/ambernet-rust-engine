use crate::Size2D;

#[derive(Debug)]
pub struct AsnWindowId(pub u64);

impl From<AsnWindowId> for u64 {
    fn from(window_id: AsnWindowId) -> Self {
        window_id.0
    }
}

impl From<u64> for AsnWindowId {
    fn from(raw_id: u64) -> Self {
        Self(raw_id)
    }
}

#[derive(Debug)]
pub enum AsnWindowEvent {
    Resized(Size2D<u32>),
    RedrawRequested,
    CloseRequested,
}

#[derive(Debug)]
pub enum AsnEvent {
    Empty,
    WindowEvent {
        window_id: AsnWindowId,
        event: AsnWindowEvent,
    },
}
