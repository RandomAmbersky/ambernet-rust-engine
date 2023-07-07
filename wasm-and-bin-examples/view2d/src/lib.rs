use asn_core::{AsnContext, AsnError, AsnEvent, AsnHandlerTrait, AsnWindowEvent};
use asn_logger::{info, AsnLogLevel};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
// #[allow(dead_code)]
extern "C" {
    #[cfg(target_arch = "wasm32")]
    fn alert(s: &str);
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn greet(name: &str) {
    #[cfg(target_arch = "wasm32")]
    {
        alert(&format!("Hello, {}!", name));
    }
    println!("Hello {:}", name);
}

struct MyHandler();

impl AsnHandlerTrait for MyHandler {
    fn proceed(&mut self, ctx: &mut AsnContext, evt: &AsnEvent) -> Option<AsnError> {
        println!("{:?}", evt);
        match evt {
            AsnEvent::WindowEvent(e) => match e {
                AsnWindowEvent::Resized(_) => None,
                AsnWindowEvent::RedrawRequested => None,
                AsnWindowEvent::CloseRequested => {
                    ctx.is_need_exit = true;
                    None
                }
            },
            _ => None,
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn start() {
    let l: AsnLogLevel = AsnLogLevel::Trace;
    asn_logger::init_log(l);
    info!("It worked :)");

    let h = MyHandler {};
    asn_runner_winit::run(h);
}
