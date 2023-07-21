use crate::runner_trait::AsnRunnerTrait;
use asn_core::AsnContext;
use winit::event_loop::{ControlFlow, EventLoop};

#[derive(Default)]
pub struct AsnRunner {
    event_loop: EventLoop<()>,
}

impl AsnRunner {
    pub fn new() -> Self {
        let event_loop = EventLoop::new();
        AsnRunner { event_loop }
    }
}

impl AsnRunner {
    pub fn run(self, ctx: AsnContext) {
        self.event_loop
            .run(move |event, _event_loop_window_target, control_flow| {
                if ctx.is_need_exit() {
                    *control_flow = ControlFlow::Exit;
                    return;
                }

                *control_flow = ControlFlow::Poll;
                // let evt = convert_event(&event);
                // if let Some(e) = evt {
                //     h.proceed(&mut ctx, &e);
                // }
            })
    }
}
