pub struct AsnContext {
    is_need_exit: bool,
}

impl AsnContext {
    pub fn new() -> Self {
        Self {
            is_need_exit: false,
        }
    }
    pub fn is_need_exit(&self) -> bool {
        self.is_need_exit
    }
    pub fn set_need_exit(&mut self) {
        self.is_need_exit = true;
    }
}
