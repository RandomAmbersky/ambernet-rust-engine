mod asn_window;
mod wgpu_context;
mod winit_context;
mod winit_event_processor;

use crate::winit_event_processor::convert_event;
use asn_core::{AsnContext, AsnHandlerTrait};
use winit::event_loop::{ControlFlow, EventLoop};

pub fn run<H: 'static>(mut h: H)
where
    H: AsnHandlerTrait,
{
    let event_loop = EventLoop::new();
    let winint_ctx = winit_context::new(&event_loop);
    let wgpu_ctx = wgpu_context::new(winint_ctx.get_window().get_window());

    let mut ctx = AsnContext::default();

    event_loop.run(move |event, _event_loop_window_target, control_flow| {
        if ctx.is_need_exit {
            *control_flow = ControlFlow::Exit;
            return;
        }

        *control_flow = ControlFlow::Poll;
        let evt = convert_event(&event);
        if let Some(e) = evt {
            h.proceed(&mut ctx, &e);
        }
    })
}
