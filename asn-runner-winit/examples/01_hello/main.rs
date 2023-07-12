mod library;

use crate::library::{get_context, get_handler};
use asn_core::{AsnWinapiTrait, Size2D};
use asn_logger::AsnLogLevel;

fn main() {
    asn_logger::init_log(AsnLogLevel::Debug);

    let mut handler = get_handler();
    let (runner, mut ctx) = get_context();
    let winapi = ctx.get_winapi();
    winapi.window_resize(&Size2D {
        width: 10,
        height: 10,
    });
    runner.run(&mut ctx, &mut handler);
}
