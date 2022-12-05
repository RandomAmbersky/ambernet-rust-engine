use crate::{AsnContext, ExtHandlerTrait};
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};

pub fn run<E>(ctx: AsnContext, event_loop: EventLoop<()>, ext: E)
where
    E: ExtHandlerTrait,
{
    event_loop.run(move |event, event_loop_window_target, control_flow| {
        *control_flow = ControlFlow::Poll;

        process_event(&ctx, &event);

        match event {
            Event::NewEvents(_) => {}
            Event::WindowEvent { window_id, event } => {
                match event {
                    WindowEvent::Resized(_) => {}
                    WindowEvent::Moved(_) => {}
                    WindowEvent::CloseRequested => {}
                    WindowEvent::Destroyed => {}
                    WindowEvent::DroppedFile(_) => {}
                    WindowEvent::HoveredFile(_) => {}
                    WindowEvent::HoveredFileCancelled => {}
                    WindowEvent::ReceivedCharacter(_) => {}
                    WindowEvent::Focused(_) => {}
                    WindowEvent::KeyboardInput { .. } => {}
                    WindowEvent::ModifiersChanged(_) => {}
                    WindowEvent::Ime(_) => {}
                    WindowEvent::CursorMoved { .. } => {}
                    WindowEvent::CursorEntered { .. } => {}
                    WindowEvent::CursorLeft { .. } => {}
                    WindowEvent::MouseWheel { .. } => {}
                    WindowEvent::MouseInput { .. } => {}
                    WindowEvent::TouchpadPressure { .. } => {}
                    WindowEvent::AxisMotion { .. } => {}
                    WindowEvent::Touch(_) => {}
                    WindowEvent::ScaleFactorChanged { .. } => {}
                    WindowEvent::ThemeChanged(_) => {}
                    WindowEvent::Occluded(_) => {}
                };
            }
            Event::DeviceEvent { .. } => {}
            Event::UserEvent(_) => {}
            Event::Suspended => {}
            Event::Resumed => {}
            Event::MainEventsCleared => {}
            Event::RedrawRequested(window_id) => {}
            Event::RedrawEventsCleared => {}
            Event::LoopDestroyed => {}
        }
    })
    // ext.draw(&ctx);
    // ext.update(&ctx);
}

fn process_event(ctx: &AsnContext, event: &Event<()>) {}
