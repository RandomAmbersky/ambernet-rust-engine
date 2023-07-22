mod handler;

use crate::library::engine::handler::Handler;
use asn_core::{AsnContext, AsnEngineTrait};
use asn_runner_winit::{run, WinapiPreset};

pub struct Engine<'a> {
    ctx: Option<&'a AsnContext>,
}

impl<'a> Engine<'a> {
    pub fn new() -> Self {
        Engine { ctx: None }
    }
}

impl<'a> AsnEngineTrait<'a> for Engine<'a> {
    type WinApi = ();
    type Scene = ();

    fn get_winapi(&mut self) -> &'a mut Self::WinApi {
        todo!()
    }

    fn get_context(&mut self) -> &'a mut AsnContext {
        todo!()
    }

    fn get_scene(&mut self) -> &'a mut Self::Scene {
        todo!()
    }

    fn run(&mut self) {
        let mut preset: WinapiPreset<Handler> = WinapiPreset::new();
        let ctx = AsnContext::new();
        let handler = Handler::new();
        preset.set_ctx(ctx);
        preset.set_handler(handler);
        run(preset);
    }
}
