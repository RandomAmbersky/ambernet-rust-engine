use asn_core::{AsnContext, AsnEngineTrait};
use asn_runner_winit::{AsnRunner, AsnRunnerTrait};
use std::ops::Deref;

pub struct Engine {
    ctx: AsnContext,
    runner: AsnRunner,
}

impl Engine {
    pub fn new() -> Self {
        let ctx = AsnContext::new();
        let runner = AsnRunner::new();
        Engine { ctx, runner }
    }
}

impl<'a> AsnEngineTrait<'a> for Engine {
    type WinApi = ();
    type Scene = ();

    fn get_winapi(&mut self) -> &'a mut Self::WinApi {
        todo!()
    }

    fn get_context(&mut self) -> &'a mut AsnContext {
        &mut self.ctx
    }

    fn get_scene(&mut self) -> &'a mut Self::Scene {
        todo!()
    }

    fn run(&mut self) {
        self.runner.run(self.ctx);
    }
}
