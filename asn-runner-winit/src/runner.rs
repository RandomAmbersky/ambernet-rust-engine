use crate::asn_winapi::AsnWgpuWinApi;
use crate::winit_event_processor::convert_event;
use asn_core::AsnError;
use asn_core::{AsnContext, AsnHandlerTrait};
use winit::event_loop::{ControlFlow, EventLoop};

pub struct Runner {
    event_loop: EventLoop<()>,
}

impl Runner {
    pub fn new() -> Self {
        let event_loop = EventLoop::new();
        Runner { event_loop }
    }
    pub fn new_winapi(&self) -> AsnWgpuWinApi {
        let winapi = AsnWgpuWinApi::new(&self.event_loop);
        winapi
    }
    pub fn run<H: 'static>(self, ctx: AsnContext<AsnWgpuWinApi>, mut h: H)
    where
        H: AsnHandlerTrait,
    {
        self.event_loop
            .run(move |event, _event_loop_window_target, control_flow| {
                if ctx.is_need_exit() {
                    *control_flow = ControlFlow::Exit;
                    return;
                }

                *control_flow = ControlFlow::Poll;
                let evt = convert_event(&event);
                if let Some(e) = evt {
                    h.proceed(&e);
                }
            })
    }
}

// pub fn run(event_loop: EventLoop<()>, mut ctx: A, mut h: H)
// where
//     W: AsnWinapiTrait,
//     A: AsnContextTrait<W>,
//     H: AsnHandlerTrait<W, A>,
// {
//     event_loop.run(move |event, _event_loop_window_target, control_flow| {
//         if ctx.is_need_exit() {
//             *control_flow = ControlFlow::Exit;
//             return;
//         }
//
//         *control_flow = ControlFlow::Poll;
//         let evt = convert_event(&event);
//         if let Some(e) = evt {
//             h.proceed(&mut ctx, &e);
//         }
//     })
// }
