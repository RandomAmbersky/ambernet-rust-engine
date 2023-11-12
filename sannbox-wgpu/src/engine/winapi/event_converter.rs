use asn_core::events::AsnEvent;
use asn_core::events::AsnKeyboardEvent::{Pressed, Released};
use asn_core::events::AsnWindowEvent::{CloseRequested, RedrawRequested, Resized};
use asn_core::math::Size2D;
use asn_logger::info;
use winit::event::{ElementState, Event, WindowEvent};
use winit::window::WindowId;

#[derive(Debug, Clone, Copy)]
pub enum CustomEvent {
    UpdateEvent,
}

pub fn convert_asn_event(e: &AsnEvent) -> CustomEvent {
    match e {
        AsnEvent::UpdateEvent => CustomEvent::UpdateEvent,
        _ => panic!("can't convert event {:?}", e),
    }
}

pub fn convert_event(e: &Event<CustomEvent>) -> Option<AsnEvent> {
    // println!("{:?}", e);
    match e {
        Event::RedrawRequested(_window_id) => Some(AsnEvent::WindowEvent(RedrawRequested)),
        Event::MainEventsCleared => Some(AsnEvent::WindowEvent(RedrawRequested)),
        Event::WindowEvent {
            ref event,
            window_id,
        } => process_window_event(window_id, event),
        Event::UserEvent(t) => process_custom_event(t),
        _ => None,
    }
}

fn process_custom_event(e: &CustomEvent) -> Option<AsnEvent> {
    match e {
        CustomEvent::UpdateEvent => Some(AsnEvent::UpdateEvent),
        _ => None,
    }
}

fn process_window_event(_window_id: &WindowId, e: &WindowEvent) -> Option<AsnEvent> {
    match e {
        WindowEvent::CloseRequested => Some(AsnEvent::WindowEvent(CloseRequested)),
        WindowEvent::Resized(size) => {
            let w_size: Size2D<u32> = Size2D {
                width: size.width,
                height: size.height,
            };
            Some(AsnEvent::WindowEvent(Resized(w_size)))
        }
        WindowEvent::ScaleFactorChanged {
            new_inner_size: size,
            ..
        } => {
            let w_size: Size2D<u32> = Size2D {
                width: size.width,
                height: size.height,
            };
            Some(AsnEvent::WindowEvent(Resized(w_size)))
        }
        WindowEvent::KeyboardInput { input, .. } => {
            // println!("WindowEvent::KeyboardInput: {:?}", input);
            match input.state {
                ElementState::Pressed => Some(AsnEvent::KeyboardEvent(Pressed(input.scancode))),
                ElementState::Released => Some(AsnEvent::KeyboardEvent(Released(input.scancode))),
            }
        }
        _ => None,
    }
}
