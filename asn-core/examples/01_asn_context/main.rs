mod library;

use crate::library::get_context;
use asn_core::{AsnWinapiTrait, Size2D};

fn main() {
    let mut ctx = get_context();
    let winapi = ctx.get_winapi();
    winapi.window_resize(&Size2D {
        width: 10,
        height: 10,
    });
}
