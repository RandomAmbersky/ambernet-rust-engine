use asn_core::traits::{TAsnBaseEngine, TAsnHandler};
use asn_logger::trace;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{EventLoop, EventLoopProxy};
use winit::window::{Window, WindowId};

mod application_handler;
mod event_converter;

pub struct RunnerDataset<'a, E, H>
where
    E: TAsnBaseEngine,
    H: TAsnHandler<E>,
{
    window: Option<Window>,
    e: &'a mut E,
    h: &'a mut H,
}

pub fn new_runner_dataset<'a, E, H>(e: &'a mut E, h: &'a mut H) -> RunnerDataset<'a, E, H>
where
    E: TAsnBaseEngine,
    H: TAsnHandler<E>,
{
    RunnerDataset { window: None, e, h }
}

pub fn run_loop<E, H>(r: &mut RunnerDataset<E, H>)
where
    E: TAsnBaseEngine,
    H: TAsnHandler<E>,
{
    trace!("run_loop:run");
    // let event_loop = EventLoop::new().unwrap();

    let event_loop = EventLoop::<Event<()>>::with_user_event().build().unwrap();

    // let event_loop_proxy: EventLoopProxy<Event<UserEvent>> = event_loop.create_proxy();
    // let evt = UserEvent::WakeUp;
    // let win_event = WindowEvent::CloseRequested;
    // event_loop_proxy.send_event(Event::WindowEvent{ window_id: WindowId.into(0), event: WindowEvent::CloseRequested }).unwrap();

    // let evt = WindowEvent::RedrawRequested;
    // event_loop_proxy.send_event(evt).expect("TODO: panic message");
    event_loop.run_app(r).unwrap()
}
