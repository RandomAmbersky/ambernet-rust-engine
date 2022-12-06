use crate::{context, AsnContext, ExtHandlerTrait};
use winit::dpi::PhysicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};

pub fn run<E>(mut ctx: AsnContext, event_loop: EventLoop<()>, ext: E)
where
    E: ExtHandlerTrait,
{
    event_loop.run(move |event, event_loop_window_target, control_flow| {
        if ctx.is_need_exit {
            *control_flow = ControlFlow::Exit;
            return;
        }

        *control_flow = ControlFlow::Poll;

        process_event(&mut ctx, &event);

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

fn process_event(ctx: &mut AsnContext, event: &Event<()>) {
    match event {
        Event::NewEvents(_) => {}
        Event::WindowEvent { window_id, event } => {
            process_window_event(ctx, event);
        }
        Event::DeviceEvent { .. } => {}
        Event::UserEvent(_) => {}
        Event::Suspended => {}
        Event::Resumed => {}
        Event::MainEventsCleared => process_main_events_cleared(ctx),
        Event::RedrawRequested(window_id) => process_redraw_requested(ctx),
        Event::RedrawEventsCleared => {}
        Event::LoopDestroyed => {}
    };
}

fn process_window_event(ctx: &mut AsnContext, event: &WindowEvent) {
    match event {
        WindowEvent::Resized(size) => {
            process_window_resized(ctx, size);
        }
        WindowEvent::Moved(_) => {}
        WindowEvent::CloseRequested => ctx.is_need_exit = true,
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
    }
}

fn process_redraw_requested(ctx: &AsnContext) {}

fn process_main_events_cleared(ctx: &AsnContext) {}

fn process_window_resized(ctx: &AsnContext, size: &PhysicalSize<u32>) {
    ctx.gfx
        .main_window
        .configure_surface(&ctx.gfx.adapter, &ctx.gfx.device);
    ctx.gfx.main_window.request_redraw();
}
