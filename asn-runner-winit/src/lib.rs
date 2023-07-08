mod winit_context;
mod winit_event_processor;

use crate::winit_context::WinitContext;
use crate::winit_event_processor::process_event;
use asn_core::AsnHandlerTrait;
use winit::event_loop::{ControlFlow, EventLoop};

pub fn run<H: 'static>(mut h: H)
where
    H: AsnHandlerTrait,
{
    let event_loop = EventLoop::new();
    let mut w_ctx = WinitContext::new(&event_loop);

    event_loop.run(move |event, _event_loop_window_target, control_flow| {
        if w_ctx.ctx.is_need_exit {
            *control_flow = ControlFlow::Exit;
            return;
        }

        *control_flow = ControlFlow::Poll;
        let evt = process_event(&mut w_ctx, &event);
        if let Some(e) = evt {
            h.proceed(&mut w_ctx.ctx, &e);
        }
    })
}
