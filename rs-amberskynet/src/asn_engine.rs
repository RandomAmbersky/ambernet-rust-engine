use crate::context::AsnContext;
use crate::{context, ExtHandlerTrait};
use winit::event::Event;
use winit::event_loop::{ControlFlow, EventLoop};

pub fn init() -> (AsnContext, EventLoop<()>) {
    let event_loop = EventLoop::new();
    let ctx = context::new(&event_loop);
    (ctx, event_loop)
}

pub fn process_event(_ctx: &AsnContext, _event: &Event<()>, _control_flow: &mut ControlFlow) {}

pub fn run<E>(ctx: AsnContext, event_loop: EventLoop<()>, ext: E)
where
    E: ExtHandlerTrait,
{
    event_loop.run(
        move |event, event_loop_window_target, control_flow| match event {
            Event::NewEvents(_) => {}
            Event::WindowEvent { .. } => {}
            Event::DeviceEvent { .. } => {}
            Event::UserEvent(_) => {}
            Event::Suspended => {}
            Event::Resumed => {}
            Event::MainEventsCleared => {}
            Event::RedrawRequested(_) => {}
            Event::RedrawEventsCleared => {}
            Event::LoopDestroyed => {}
        },
    )
    // ext.draw(&ctx);
    // ext.update(&ctx);
}
