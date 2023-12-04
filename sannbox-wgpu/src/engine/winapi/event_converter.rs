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
    // info!("{:?}", e);
    match e {
        Event::DeviceEvent { event, device_id } => {
            info!("{:?} {:?}", event, device_id);
            None
        }
        // Event::MainEventsCleared => Some(AsnEvent::WindowEvent(RedrawRequested)),
        Event::WindowEvent {
            ref event,
            window_id,
        } => process_window_event(window_id, event),
        Event::UserEvent(t) => process_custom_event(t),
        // Event::RedrawEventsCleared => None,
        Event::NewEvents(_e) => None,
        Event::AboutToWait => None,
        _ => {
            info!("{:?}", e);
            None
        }
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
        WindowEvent::RedrawRequested => Some(AsnEvent::WindowEvent(RedrawRequested)),
        // WindowEvent::ScaleFactorChanged {
        //     scale_factor,
        //     inner_size_writer,
        // } => {
        //     info!(
        //         "ScaleFactorChanged {:?} {:?}",
        //         scale_factor, inner_size_writer
        //     );
        //     let mut inner_size;
        //     inner_size_writer.new_inner_size;
        //     None
        // }
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
            // info!("WindowEvent::KeyboardInput: {:?}", event);
            // match event.state {
            //     ElementState::Pressed => Some(AsnEvent::KeyboardEvent(Pressed(event.scancode))),
            //     ElementState::Released => Some(AsnEvent::KeyboardEvent(Released(event.scancode))),
            // }
            None
        }
        WindowEvent::CursorMoved { .. } => None,
        WindowEvent::CursorEntered { .. } => None,
        WindowEvent::CursorLeft { .. } => None,
        WindowEvent::Focused(_f) => None,
        WindowEvent::Occluded(_f) => None,
        _ => {
            info!("WindowEvent {:?}", e);
            None
        }
    }
}
