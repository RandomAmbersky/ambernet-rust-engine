mod library;

use asn_core::AsnEngineTrait;
use library::get_engine;

fn main() {
    let mut e = get_engine();
    let w = e.get_winapi();
    e.run();
}
