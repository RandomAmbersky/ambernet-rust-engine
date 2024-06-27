use crate::math::Size2D;

#[derive(Debug)]
pub enum AsnWindowEvent {
    Resized(Size2D<u32>),
    RedrawRequested,
    CloseRequested,
}

#[derive(Debug)]
pub enum AsnKeyboardEvent {
    Pressed(u32),  // scancode
    Released(u32), // scancode
}

#[derive(Debug)]
pub enum AsnEvent {
    Empty,
    UpdateEvent,
    WindowEvent(AsnWindowEvent),
    KeyboardEvent(AsnKeyboardEvent),
}

pub trait AsnEventEmitter {
    fn emit(&mut self, e: &AsnEvent) -> Result<(), String>;
    fn pull(&mut self) -> Option<AsnEvent>;
}
