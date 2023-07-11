pub mod asn_winapi;
// mod winit_event_processor;

use winit::event_loop::EventLoop;
pub use asn_winapi::{winapi_new, AsnWgpuWinApi};

pub struct Runner {
	winapi: AsnWgpuWinApi,
	event_loop: EventLoop<()>
}

pub fn new() -> Runner {
	let event_loop = EventLoop::new();
	let winapi = asn_winapi::winapi_new(&event_loop);
	Runner {
		event_loop,
		winapi
	}
}

// use crate::winit_event_processor::convert_event;
// use asn_core::{AsnContextTrait, AsnHandlerTrait, AsnWinapiTrait};
// use winit::event_loop::{ControlFlow, EventLoop};
//
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
