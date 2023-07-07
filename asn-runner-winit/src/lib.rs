mod winit_event_processor;

use crate::winit_event_processor::process_event;
use asn_core::{AsnContext, AsnHandlerTrait};
use winit::event_loop::{ControlFlow, EventLoop};

pub fn run<H: 'static>(mut h: H)
where
    H: AsnHandlerTrait,
{
    let event_loop = EventLoop::new();
    let mut ctx = AsnContext::default();

    event_loop.run(move |event, _event_loop_window_target, control_flow| {
        if ctx.is_need_exit {
            *control_flow = ControlFlow::Exit;
            return;
        }

        *control_flow = ControlFlow::Poll;
        let evt = process_event(&mut ctx, &event);
        if let Some(..) = evt {
            h.proceed(&mut ctx, &evt.unwrap());
        }
    })
}
