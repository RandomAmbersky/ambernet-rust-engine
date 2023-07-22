use crate::asn_winapi::AsnWgpuWinApi;
use crate::winit_event_processor::convert_event;
use asn_core::{AsnApiTrait, AsnBaseContextTrait, AsnContext, AsnHandlerTrait};
use winit::event_loop::{ControlFlow, EventLoop};

#[derive(Default)]
pub struct WinapiPreset<H>
where
    H: AsnHandlerTrait,
{
    event_loop: Option<EventLoop<()>>,
    ctx: Option<AsnContext>,
    winapi: Option<AsnWgpuWinApi>,
    handler: Option<H>,
}

impl<H> WinapiPreset<H>
where
    H: AsnHandlerTrait,
{
    pub fn new() -> Self {
        let event_loop = EventLoop::new();
        let winapi = AsnWgpuWinApi::new(&event_loop);
        WinapiPreset {
            event_loop: Some(event_loop),
            winapi: Some(winapi),
            ctx: None,
            handler: None,
        }
    }
    pub fn set_ctx(&mut self, ctx: AsnContext) {
        self.ctx = Some(ctx)
    }
    pub fn set_handler(&mut self, handler: H) {
        self.handler = Some(handler)
    }
}

pub fn run<H: 'static>(mut preset: WinapiPreset<H>)
where
    H: AsnHandlerTrait,
{
    let event_loop = preset.event_loop.take().unwrap();
    let mut ctx = preset.ctx.take().unwrap();
    let mut hanlder = preset.handler.take().unwrap();
    let mut winapi = preset.winapi.take().unwrap();

    event_loop.run(move |event, _event_loop_window_target, control_flow| {
        *control_flow = ControlFlow::Poll;

        if ctx.is_need_exit() {
            *control_flow = ControlFlow::Exit;
            return;
        }

        let evt = convert_event(&event);
        if let Some(e) = evt {
            hanlder.proceed(&mut ctx, &e);
        }
    })
}
