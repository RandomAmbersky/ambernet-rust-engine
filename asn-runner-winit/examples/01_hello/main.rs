mod my_context;
mod my_handler;

use asn_core::{AsnError, AsnEvent, AsnHandlerTrait, AsnWindowEvent};
use asn_runner_winit::winapi_new;
use my_handler::MyHandler;
use winit::event_loop::EventLoop;

fn main() {
    let event_loop = EventLoop::new();
    let win_api = winapi_new(&event_loop);
    let ctx = my_context::new(win_api);
    let h = MyHandler {};
    asn_runner_winit::run(event_loop, ctx, h);
}
