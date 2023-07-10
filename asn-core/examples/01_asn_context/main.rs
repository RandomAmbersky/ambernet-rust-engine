mod library;

use crate::library::{get_context, WinApiTrait};

fn main() {
    let mut ctx = get_context();
    let mut winapi = ctx.get_winapi();
    winapi.resize(10, 10);
}
