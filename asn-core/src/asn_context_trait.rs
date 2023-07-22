pub trait AsnBaseContextTrait {
    fn is_need_exit(&self) -> bool;
    fn set_need_exit(&mut self);
}

pub trait AsnApiTrait<'a> {
    type WinApi;
    fn get_winapi() -> &'a mut Self::WinApi;
}
