pub use crate::asn_context_trait::{AsnApiTrait, AsnBaseContextTrait};

#[derive(Default)]
pub struct AsnContext {
    is_need_exit: bool,
}

impl AsnContext {
    pub fn new() -> Self {
        Self {
            is_need_exit: false,
        }
    }
}

impl AsnBaseContextTrait for AsnContext {
    fn is_need_exit(&self) -> bool {
        self.is_need_exit
    }
    fn set_need_exit(&mut self) {
        self.is_need_exit = true
    }
}
