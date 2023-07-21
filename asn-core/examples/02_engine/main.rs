mod library;

use asn_core::AsnEngineTrait;
use library::get_engine;

fn main() {
    let mut e = get_engine();
    e.run();
}
