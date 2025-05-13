#[derive(Debug, Clone)]
pub enum AsnWindowEvent {
    None,
    // Resized(Size2D<u32>),
    RedrawRequested,
    CloseRequested,
}

#[derive(Debug, Clone)]
pub enum AsnKeyboardEvent {
    Pressed(u32),  // scancode
    Released(u32), // scancode
}

#[derive(Debug, Clone)]
pub enum AsnEvent {
    Empty,
    AppExit,
    UpdateEvent,
    WindowEvent(AsnWindowEvent),
    KeyboardEvent(AsnKeyboardEvent),
}
