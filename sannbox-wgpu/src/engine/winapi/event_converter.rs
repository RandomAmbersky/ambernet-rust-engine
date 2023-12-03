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
    match e {
        Event::DeviceEvent { event, device_id } => {
            info!("{:?} {:?}", event, device_id);
            None
        }
        // Event::RedrawRequested(_window_id) => Some(AsnEvent::WindowEvent(RedrawRequested)),
        // Event::MainEventsCleared => Some(AsnEvent::WindowEvent(RedrawRequested)),
        Event::WindowEvent {
            ref event,
            window_id,
        } => process_window_event(window_id, event),
        Event::UserEvent(t) => process_custom_event(t),
        // Event::RedrawEventsCleared => None,
        Event::NewEvents(e) => None,
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
        // WindowEvent::ScaleFactorChanged {
        //     inner_size_writer: size,
        //     ..
        // } => {
        //     info!("{:?}", e);
        //     let w_size: Size2D<u32> = Size2D {
        //         width: size.width,
        //         height: size.height,
        //     };
        //     Some(AsnEvent::WindowEvent(Resized(w_size)))
        // }
        WindowEvent::KeyboardInput { event, .. } => {
            // println!("WindowEvent::KeyboardInput: {:?}", input);
            info!("WindowEvent::KeyboardInput: {:?}", event);
            // match event.state {
            //     ElementState::Pressed => Some(AsnEvent::KeyboardEvent(Pressed(event.scancode))),
            //     ElementState::Released => Some(AsnEvent::KeyboardEvent(Released(event.scancode))),
            // }
            None
        }
        _ => None,
    }
}
