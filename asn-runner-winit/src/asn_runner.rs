use crate::winit_event_processor::convert_event;
use asn_core::{AsnContext, AsnHandlerTrait};
use winit::event_loop::{ControlFlow, EventLoop};

#[derive(Default)]
pub struct WinapiPreset<H>
where
    H: AsnHandlerTrait,
{
    event_loop: Option<EventLoop<()>>,
    ctx: Option<AsnContext>,
    handler: Option<H>,
}

impl<H> WinapiPreset<H>
where
    H: AsnHandlerTrait,
{
    pub fn new() -> Self {
        let event_loop = Some(EventLoop::new());
        WinapiPreset {
            event_loop,
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
    let mut ctx = preset.ctx.unwrap();
    let mut hanlder = preset.handler.unwrap();

    event_loop.run(move |event, _event_loop_window_target, control_flow| {
        if ctx.is_need_exit() {
            *control_flow = ControlFlow::Exit;
            return;
        }

        *control_flow = ControlFlow::Poll;
        let evt = convert_event(&event);
        if let Some(e) = evt {
            hanlder.proceed(&mut ctx, &e);
        }
    })
}
