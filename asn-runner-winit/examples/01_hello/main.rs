mod library;

use crate::library::{get_context, get_handler};
use asn_core::{AsnWinapiTrait, Size2D};

fn main() {
    let handler = get_handler();
    let (runner, mut ctx) = get_context();
    let winapi = ctx.get_winapi();
    winapi.window_resize(Size2D {
        width: 10,
        height: 10,
    });
    ctx.set_need_exit();
    runner.run(ctx, handler);
}
