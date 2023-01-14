use crate::core_gfx::gfx_context::GfxContextTrait;
use crate::{AsnContext, ExtHandlerTrait};
use winit::dpi::PhysicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};

pub fn run<E: 'static>(mut ctx: AsnContext, event_loop: EventLoop<()>, mut ext: E)
where
    E: ExtHandlerTrait,
{
    event_loop.run(move |event, _event_loop_window_target, control_flow| {
        if ctx.is_need_exit {
            *control_flow = ControlFlow::Exit;
            return;
        }

        *control_flow = ControlFlow::Poll;

        process_event(&mut ctx, &event);

        match event {
            Event::NewEvents(_) => {}
            Event::WindowEvent { .. } => {}
            Event::DeviceEvent { .. } => {}
            Event::UserEvent(_) => {}
            Event::Suspended => {}
            Event::Resumed => {}
            Event::MainEventsCleared => {
                ext.update(&mut ctx);
                match ctx.gfx.begin_frame() {
                    Ok(_) => {}
                    Err(_) => {
                        println!("Error");
                    }
                }
                ext.draw(&mut ctx);
                match ctx.gfx.end_frame() {
                    Ok(_) => {}
                    Err(_) => {
                        println!("Error");
                    }
                }
            }
            Event::RedrawRequested(_) => {}
            Event::RedrawEventsCleared => {}
            Event::LoopDestroyed => {}
        }
    })
}

fn process_event(ctx: &mut AsnContext, event: &Event<()>) {
    match event {
        Event::NewEvents(_) => {}
        Event::WindowEvent {
            window_id: _,
            event,
        } => {
            process_window_event(ctx, event);
        }
        Event::DeviceEvent { .. } => {}
        Event::UserEvent(_) => {}
        Event::Suspended => {}
        Event::Resumed => {}
        Event::MainEventsCleared => process_main_events_cleared(ctx),
        Event::RedrawRequested(_window_id) => process_redraw_requested(ctx),
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
        WindowEvent::ScaleFactorChanged {
            scale_factor: _,
            new_inner_size,
        } => process_window_resized(ctx, new_inner_size),
        WindowEvent::ThemeChanged(_) => {}
        WindowEvent::Occluded(_) => {}
    }
}

fn process_main_events_cleared(ctx: &AsnContext) {
    ctx.gfx.main_window.request_redraw();
}

fn process_window_resized(ctx: &AsnContext, _size: &PhysicalSize<u32>) {
    ctx.gfx
        .main_window
        .configure_surface(&ctx.gfx.adapter, &ctx.gfx.device);
    ctx.gfx.main_window.request_redraw();
}

fn process_redraw_requested(_ctx: &AsnContext) {}
