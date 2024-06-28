use asn_core::events::{AsnEvent, AsnEventEmitter};
use asn_core::traits::{TAsnBaseEngine, TAsnHandler};
use asn_winit_released::run_loop;
use std::collections::VecDeque;

#[derive(Default)]
struct Engine {
    exit: bool,
    events: VecDeque<AsnEvent>,
}
struct Handler {}

impl TAsnBaseEngine for Engine {
    fn is_need_exit(&self) -> bool {
        self.exit
    }
    fn set_need_exit(&mut self) {
        self.exit = true
    }
}

impl AsnEventEmitter for Engine {
    fn emit(&mut self, e: AsnEvent) -> Result<(), String> {
        println!("Emit {:?}", e);
        self.events.push_back(e);
        Ok(())
    }

    fn pull(&mut self) -> Option<AsnEvent> {
        let e = self.events.pop_front();
        if e.is_some() {
            println!("Emit {:?}", e);
        }
        e
    }
}

impl TAsnHandler<Engine> for Handler {
    fn handle(&mut self, evt: &AsnEvent, engine: &mut Engine) {
        // println!("Handle {:?}", evt);
    }
}

fn main() {
    let mut e = Engine::default();
    let mut h = Handler {};
    run_loop(&mut e, &mut h);
}
