use crate::event_converter::decode_asn_event;
use asn_core::events::AsnEvent;
use asn_core::traits::{TAsnBaseEngine, TAsnHandler};
use asn_logger::trace;
use std::borrow::BorrowMut;
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
    event_loop_proxy: Option<EventLoopProxy<Event<()>>>,
    e: &'a mut E,
    h: &'a mut H,
}

pub fn new_runner_dataset<'a, E, H>(e: &'a mut E, h: &'a mut H) -> RunnerDataset<'a, E, H>
where
    E: TAsnBaseEngine,
    H: TAsnHandler<E>,
{
    RunnerDataset {
        event_loop_proxy: None,
        window: None,
        e,
        h,
    }
}

pub fn run_loop<E, H>(e: &mut E, h: &mut H)
where
    E: TAsnBaseEngine,
    H: TAsnHandler<E>,
{
    trace!("run_loop:run");
    let event_loop = EventLoop::<Event<()>>::with_user_event().build().unwrap();

    let event_loop_proxy: EventLoopProxy<Event<()>> = event_loop.create_proxy();

    let mut r = new_runner_dataset(e, h);
    r.event_loop_proxy = Some(event_loop_proxy);
    // event_loop_proxy.send_event(evt).expect("TODO: panic message");
    event_loop.run_app(&mut r).unwrap()
}

pub trait AsnEventLoop<E, H>
where
    E: TAsnBaseEngine,
    H: TAsnHandler<E>,
{
    fn emit(&mut self, e: &AsnEvent) -> Result<(), String>;
    fn run_loop(&mut self, e: &mut E, h: &mut H) -> Result<(), String>;
}

struct AsnLoop {
    event_loop: Option<EventLoop<Event<()>>>,
    event_loop_proxy: Option<EventLoopProxy<Event<()>>>,
}

impl AsnLoop {
    pub fn new() -> Self {
        let event_loop = EventLoop::<Event<()>>::with_user_event().build().unwrap();
        let event_loop_proxy: EventLoopProxy<Event<()>> = event_loop.create_proxy();
        AsnLoop {
            event_loop: Some(event_loop),
            event_loop_proxy: Some(event_loop_proxy),
        }
    }
}

impl<E, H> AsnEventLoop<E, H> for AsnLoop
where
    E: TAsnBaseEngine,
    H: TAsnHandler<E>,
{
    fn emit(&mut self, e: &AsnEvent) -> Result<(), String> {
        let evt = decode_asn_event(e).unwrap();
        self.event_loop_proxy
            .as_mut()
            .unwrap()
            .send_event(evt)
            .unwrap();
        Ok(())
    }

    fn run_loop(&mut self, e: &mut E, h: &mut H) -> Result<(), String> {
        trace!("run_loop:run");

        let mut r = new_runner_dataset(e, h);

        let event_loop = self.event_loop.take().unwrap();
        event_loop.run_app(&mut r).unwrap();
        Ok(())
    }
}
