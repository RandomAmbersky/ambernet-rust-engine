use winit::event_loop::{ControlFlow, EventLoop};

pub struct EventRunner {}

impl EventRunner {
    pub fn new() -> Self {
        EventRunner {}
    }
}

pub fn run() -> ! {
    let event_loop = EventLoop::new();
    event_loop.run(move |event, _event_loop_window_target, control_flow| {
        *control_flow = ControlFlow::Poll;

        // if ctx.is_need_exit {
        //     *control_flow = ControlFlow::Exit;
        //     return;
        // }

        // process_event(&mut ctx, &event);

        // match event {
        //     Event::NewEvents(_) => {}
        //     Event::WindowEvent { .. } => {}
        //     Event::DeviceEvent { .. } => {}
        //     Event::UserEvent(_) => {}
        //     Event::Suspended => {}
        //     Event::Resumed => {}
        //     Event::MainEventsCleared => {
        //         ext.update(&mut ctx);
        //         match ctx.gfx.begin_frame() {
        //             Ok(_) => {}
        //             Err(_) => {
        //                 println!("Error");
        //             }
        //         }
        //         ext.draw(&mut ctx);
        //         match ctx.gfx.end_frame() {
        //             Ok(_) => {}
        //             Err(_) => {
        //                 println!("Error");
        //             }
        //         }
        //     }
        //     Event::RedrawRequested(_) => {}
        //     Event::RedrawEventsCleared => {}
        //     Event::LoopDestroyed => {}
        // }
    })
}
