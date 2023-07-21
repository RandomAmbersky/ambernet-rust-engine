use crate::runner_trait::AsnRunnerTrait;

pub struct AsnRunner {}

impl AsnRunner {
    pub fn new() -> Self {
        AsnRunner {}
    }
}

impl AsnRunnerTrait for AsnRunner {
    type AsnContext = ();
    type AsnHandler = ();

    fn run(&mut self) {
        println!("AsnRunner: run()")
    }
}
