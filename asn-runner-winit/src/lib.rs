// pub fn run<W, A: 'static, H: 'static>(event_loop: EventLoop<()>, mut ctx: A, mut h: H)
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

mod asn_winapi;
mod runner;
mod winit_event_processor;

pub type WinApi = asn_winapi::AsnWgpuWinApi;
pub use runner::Runner;
