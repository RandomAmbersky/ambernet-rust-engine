mod library;

use crate::library::{get_context, get_handler};

use asn_logger::AsnLogLevel;
use asn_scenegraph_core::AsnScenegraphTrait;

fn main() {
    asn_logger::init_log(AsnLogLevel::Debug);

    let (runner, ctx, scene) = get_context();
    let handler = get_handler();
    runner.run(ctx, handler);
}
