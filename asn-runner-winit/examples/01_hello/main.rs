mod library;

use crate::library::{get_context, get_handler};
use asn_core::{AsnWinapiTrait, Size2D};
use asn_logger::AsnLogLevel;

fn main() {
    asn_logger::init_log(AsnLogLevel::Debug);

    let handler = get_handler();
    let (runner, mut ctx) = get_context();
    runner.run(ctx, handler);
}
