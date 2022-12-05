use rs_amberskynet::ExtHandlerTrait;

struct Handler {}
impl Handler {
    pub fn new() -> Self {
        Self {}
    }
}
impl ExtHandlerTrait for Handler {
    fn draw(&self, e: &rs_amberskynet::AsnEngine) {
        // todo!()
    }

    fn update(&self, e: &rs_amberskynet::AsnEngine) {
        // todo!()
    }
}

pub fn main() {
    let e = rs_amberskynet::new();
    let h = Handler::new();
    rs_amberskynet::run(&e, &h)
}
