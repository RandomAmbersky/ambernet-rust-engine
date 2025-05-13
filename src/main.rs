extern crate asn_core;
extern crate tokio_bus;

use asn_core::asn_event::AsnEvent;
use asn_core_bus::{AsnBus, AsnReceiver, AsnTransmitter};
use tokio_bus::new_tokio_bus;

fn main() {
    println!("Hello, world!");

    let bus = new_tokio_bus::<AsnEvent>();

    let s = bus.get_sender();
    let mut r = bus.get_receiver();

    s.send_message(AsnEvent::AppExit).unwrap();
    s.send_message(AsnEvent::UpdateEvent).unwrap();

    let mut result = r.get_message().unwrap();
    println!("result: {:?}", result);
    result = r.get_message().unwrap();
    println!("result: {:?}", result);
}
