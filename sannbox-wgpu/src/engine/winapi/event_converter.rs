use crate::engine::core::events::AsnEvent;
use crate::engine::core::events::AsnWindowEvent::{CloseRequested, RedrawRequested, Resized};
use crate::engine::core::math::Size2D;
use winit::event::{Event, WindowEvent};
use winit::window::WindowId;
use asn_logger::info;

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
        _ => None,
    }
}
