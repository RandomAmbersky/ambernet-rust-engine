use crate::asn_winapi::AsnWgpuWinApi;
use asn_core::AsnContext;
use winit::event_loop::{ControlFlow, EventLoop};

pub struct RunnerBuilder {
    event_loop: Option<EventLoop<()>>,
}

impl RunnerBuilder {
    pub fn new() -> Self {
        let event_loop = EventLoop::new();
        RunnerBuilder {
            event_loop: Some(event_loop),
        }
    }
    pub fn new_winapi(&self) -> AsnWgpuWinApi {
        let event_loop = self.event_loop.as_ref().unwrap();
        let winapi = AsnWgpuWinApi::new(event_loop);
        winapi
    }
    pub fn build(&mut self) -> Runner {
        let event_loop = self.event_loop.take();
        Runner { event_loop }
    }
}

pub struct Runner {
    event_loop: Option<EventLoop<()>>,
}

impl Runner {
    pub fn run<H>(mut self, ctx: &mut AsnContext<AsnWgpuWinApi>, h: &mut H) {
        let event_loop = self.event_loop.take().unwrap();
        event_loop.run(move |event, _event_loop_window_target, control_flow| {
            // if ctx.is_need_exit() {
            //     *control_flow = ControlFlow::Exit;
            //     return;
            // }
            //
            //         *control_flow = ControlFlow::Poll;
            //         let evt = convert_event(&event);
            //         if let Some(e) = evt {
            //             h.proceed(&mut ctx, &e);
            //         }
        })
    }
}
