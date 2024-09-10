use asn_core::events::{AsnEvent, AsnWindowEvent};
use winit::event::WindowEvent;
use winit::window::WindowId;

pub fn convert_window_event_to_asn_event(
    _window_id: WindowId,
    event: WindowEvent,
) -> Option<AsnEvent> {
    match event {
        WindowEvent::CloseRequested => Some(AsnEvent::WindowEvent(AsnWindowEvent::CloseRequested)),
        WindowEvent::RedrawRequested => {
            Some(AsnEvent::WindowEvent(AsnWindowEvent::RedrawRequested))
        }
        _ => None,
    }
}

// WindowEvent::ActivationTokenDone { .. } => None,
//   WindowEvent::Resized(_) => None,
//   WindowEvent::Moved(_) => None,
//   WindowEvent::Destroyed => None,
//   WindowEvent::DroppedFile(_) => None,
//   WindowEvent::HoveredFile(_) => None,
//   WindowEvent::HoveredFileCancelled => None,
//   WindowEvent::Focused(_) => None,
//   WindowEvent::KeyboardInput { .. } => None,
//   WindowEvent::ModifiersChanged(_) => None,
//   WindowEvent::Ime(_) => None,
//   WindowEvent::CursorMoved { .. } => None,
//   WindowEvent::CursorEntered { .. } => None,
//   WindowEvent::CursorLeft { .. } => None,
//   WindowEvent::MouseWheel { .. } => None,
//   WindowEvent::MouseInput { .. } => None,
//   WindowEvent::PinchGesture { .. } => None,
//   WindowEvent::PanGesture { .. } => None,
//   WindowEvent::DoubleTapGesture { .. } => None,
//   WindowEvent::RotationGesture { .. } => None,
//   WindowEvent::TouchpadPressure { .. } => None,
//   WindowEvent::AxisMotion { .. } => None,
//   WindowEvent::Touch(_) => None,
//   WindowEvent::ScaleFactorChanged { .. } => None,
//   WindowEvent::ThemeChanged(_) => None,
//   WindowEvent::Occluded(_) => None,
// }
