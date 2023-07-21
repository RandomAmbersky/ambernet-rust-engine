use asn_core::AsnEngineTrait;

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

    fn get_winapi() -> &'a mut Self::WinApi {
        todo!()
    }

    fn get_context() -> &'a mut Self::Context {
        todo!()
    }

    fn get_scene() -> &'a mut Self::Scene {
        todo!()
    }

    fn run() {
        todo!()
    }
}
