use asn_core::asn_event::AsnEvent;
use asn_core_bus::{AsnBus, AsnReceiver, AsnTransmitter};
use tokio_bus::new_tokio_bus;

#[test]
fn test_bus_message_passing() {
    println!("Testing bus message passing...");

    let bus = new_tokio_bus::<AsnEvent>();

    let s = bus.get_sender();
    let mut r = bus.get_receiver();

    s.send_message(AsnEvent::AppExit).unwrap();
    s.send_message(AsnEvent::UpdateEvent).unwrap();

    assert!(matches!(r.get_message().unwrap(), AsnEvent::AppExit));
    assert!(matches!(r.get_message().unwrap(), AsnEvent::UpdateEvent));
}
