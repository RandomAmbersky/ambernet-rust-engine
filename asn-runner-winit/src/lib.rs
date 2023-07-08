mod asn_winapi;
mod winit_event_processor;

use crate::asn_winapi::AsnWgpuWinApi;
use crate::winit_event_processor::convert_event;
use asn_core::{AsnContextTrait, AsnEvent, AsnHandlerTrait};
use winit::event_loop::{ControlFlow, EventLoop};

pub fn run<A: 'static, H: 'static>(mut ctx: A, mut h: H)
where
    A: AsnContextTrait,
    H: AsnHandlerTrait<A>,
{
    let event_loop = EventLoop::new();
    let win_api = asn_winapi::new(&event_loop);

    event_loop.run(move |event, _event_loop_window_target, control_flow| {
        if ctx.is_need_exit() {
            *control_flow = ControlFlow::Exit;
            return;
        }

        *control_flow = ControlFlow::Poll;
        let evt = convert_event(&event);
        if let Some(e) = evt {
            proceed(&win_api, &e);
            h.proceed(&mut ctx, &e);
        }
    })
}

fn proceed(win_api: &AsnWgpuWinApi, evt: &AsnEvent) {}
