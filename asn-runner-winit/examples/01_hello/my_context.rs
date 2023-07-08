use asn_core::AsnContextTrait;

#[derive(Default)]
pub struct MyCtx {
    is_need_exit: bool,
}

impl AsnContextTrait for MyCtx {
    fn is_need_exit(&self) -> bool {
        self.is_need_exit
    }

    fn set_need_exit(&mut self) {
        self.is_need_exit = true
    }
}
