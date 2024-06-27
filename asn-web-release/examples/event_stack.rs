use asn_core::events::{AsnEvent, AsnEventEmitter};
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct EventList {
    events: VecDeque<AsnEvent>,
}

fn new_event_list() -> EventList {
    EventList {
        events: VecDeque::new(),
    }
}

impl AsnEventEmitter for EventList {
    fn emit(&mut self, e: AsnEvent) -> Result<(), String> {
        // println!("Emit {:?}", e);
        self.events.push_back(e);
        Ok(())
    }

    fn pull(&mut self) -> Option<AsnEvent> {
        // println!("Pull event...");
        self.events.pop_front()
    }
}

fn main() {
    println!("Hello events!");
    let mut e = new_event_list();
    let shared_base = Arc::new(Mutex::new(e));
    let shared_e = shared_base.clone();

    thread::spawn(move || {
        let shared_list = shared_e.clone();
        for i in 1..10 {
            let e = shared_list.lock().unwrap().emit(AsnEvent::Empty);
            println!("spawn {i} event");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..10 {
        thread::sleep(Duration::from_millis(2));
        let evt = shared_base.lock().unwrap().pull().unwrap();
        println!("hi {:?} from the main thread!", evt);
    }
}
