use asn_core::events::{AsnEvent, AsnWindowEvent};
use asn_core::math::Size2D;
use asn_logger::info;
use winit::event::{Event, WindowEvent};
use winit::window::WindowId;

pub fn convert_event(e: &Event<()>) -> Option<AsnEvent> {
    // info!("{:?}", e);
    match e {
        Event::DeviceEvent { event, device_id } => {
            // info!("{:?} {:?}", event, device_id);
            None
        }
        // Event::MainEventsCleared => Some(AsnEvent::WindowEvent(RedrawRequested)),
        Event::WindowEvent {
            ref event,
            window_id,
        } => process_window_event(window_id, event),
        // Event::UserEvent(t) => process_custom_event(t),
        // Event::RedrawEventsCleared => None,
        Event::NewEvents(_e) => None,
        // Event::AboutToWait => Some(AsnEvent::WindowEvent(AsnWindowEvent::RedrawRequested)),
        Event::Resumed => None,
        _ => {
            // info!("{:?}", e);
            None
        }
    }
}

// fn process_custom_event(e: &CustomEvent) -> Option<AsnEvent> {
//     match e {
//         CustomEvent::UpdateEvent => Some(AsnEvent::UpdateEvent),
//         _ => None,
//     }
// }

fn process_window_event(_window_id: &WindowId, e: &WindowEvent) -> Option<AsnEvent> {
    // info!("WindowEvent {:?}", e);
    match e {
        WindowEvent::CloseRequested => Some(AsnEvent::WindowEvent(AsnWindowEvent::CloseRequested)),
        WindowEvent::Resized(size) => {
            let w_size: Size2D<u32> = Size2D {
                width: size.width,
                height: size.height,
            };
            Some(AsnEvent::WindowEvent(AsnWindowEvent::Resized(w_size)))
        }
        WindowEvent::RedrawRequested => {
            // info!(
            //     "{:?}",
            //     AsnEvent::WindowEvent(AsnWindowEvent::RedrawRequested)
            // );
            Some(AsnEvent::WindowEvent(AsnWindowEvent::RedrawRequested))
        }
        // WindowEvent::ScaleFactorChanged { .. } => Some(AsnEvent::WindowEvent(RedrawRequested)),
        //     let w_size = window.inner_size();
        //     None
        //     // state.resize(window.inner_size());
        // }
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
            // info!("WindowEvent {:?}", e);
            None
        }
    }
}
