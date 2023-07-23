use winit::event_loop::{ControlFlow, EventLoop};

pub fn run() {
    let event_loop = EventLoop::new();

    event_loop.run(move |event, _event_loop_window_target, control_flow| {
        *control_flow = ControlFlow::Poll;

        // if ctx.is_need_exit() {
        //     *control_flow = ControlFlow::Exit;
        //     return;
        // }

        // let evt = convert_event(&event);
        // if let Some(e) = evt {
        //     hanlder.proceed(&mut ctx, &e);
        // }
    })
}
