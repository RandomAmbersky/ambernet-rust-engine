extern crate asn_core;
extern crate asn_core_bus;
extern crate module_pool;

use asn_core::asn_event::AsnEvent;
use asn_core_bus::{AsnModule, AsnModulePool, AsnReceiver, AsnTransmitter};
use module_pool::new_module_pool;
use tokio_bus::new_tokio_bus;

struct Module1 {}

impl AsnModule for Module1 {
    fn init(
        &self,
        _t: impl AsnTransmitter<AsnEvent>,
        _r: impl AsnReceiver<AsnEvent>,
    ) -> Result<(), String> {
        Ok(())
    }

    fn run(&self) -> Result<(), String> {
        Ok(())
    }
}

#[test]
fn test_bus_message_passing() {
    println!("Testing bus message passing...");

    let bus = new_tokio_bus();
    let pool = new_module_pool(bus);

    let mod1 = Module1 {};
    let mod2 = Module1 {};
    let mod3 = Module1 {};

    pool.add_module(mod1).unwrap();
    pool.add_module(mod2).unwrap();
    pool.add_module(mod3).unwrap();

    // assert!(matches!(r.get_message().unwrap(), AsnEvent::AppExit));
    // assert!(matches!(r.get_message().unwrap(), AsnEvent::UpdateEvent));
}
