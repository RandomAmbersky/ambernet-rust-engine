mod asn_window;

use crate::engine::event_runner::asn_window::AsnWindow;
use winit::event_loop::{ControlFlow, EventLoop};

pub struct EventRunner {
    pub event_loop: Option<EventLoop<()>>,
    pub window: Option<AsnWindow>,
}

impl EventRunner {
    pub fn new() -> Self {
        let event_loop = EventLoop::new();
        let window = AsnWindow::new(&event_loop);
        EventRunner {
            event_loop: Some(event_loop),
            window: Some(window),
        }
    }
}

pub fn run(mut e: EventRunner) {
    let event_loop = e.event_loop.take().unwrap();

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
