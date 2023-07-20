mod library;

use crate::library::{get_context, get_handler};

use asn_logger::AsnLogLevel;

fn main() {
    asn_logger::init_log(AsnLogLevel::Debug);

    let handler = get_handler();
    let (runner, ctx) = get_context();
    runner.run(ctx, handler);
}
