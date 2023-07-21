use asn_core::AsnEngineTrait;
use asn_logger::info;

pub struct Engine {}

impl Engine {
    pub fn new() -> Self {
        Engine {}
    }
}

impl<'a> AsnEngineTrait<'a> for Engine {
    type WinApi = ();
    type Context = ();
    type Scene = ();

    fn get_winapi(&mut self) -> &'a mut Self::WinApi {
        todo!()
    }

    fn get_context(&mut self) -> &'a mut Self::Context {
        todo!()
    }

    fn get_scene(&mut self) -> &'a mut Self::Scene {
        todo!()
    }

    fn run(&mut self) {
        info!("Engine run")
    }
}
