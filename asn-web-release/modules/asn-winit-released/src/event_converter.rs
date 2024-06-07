use asn_core::events::{AsnEvent, AsnWindowEvent};
use asn_core::math::Size2D;
use asn_logger::info;
use winit::event::WindowEvent;
use winit::window::WindowId;

pub fn process_window_event(_window_id: &WindowId, e: &WindowEvent) -> Option<AsnEvent> {
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
