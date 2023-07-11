use crate::AsnWinapiTrait;

pub struct AsnContext<W>
where
    W: AsnWinapiTrait,
{
    is_need_exit: bool,
    winapi: W,
}

impl<W> AsnContext<W>
where
    W: AsnWinapiTrait,
{
    pub fn new(winapi: W) -> Self {
        Self {
            winapi,
            is_need_exit: false,
        }
    }
    fn is_need_exit(&self) -> bool {
        self.is_need_exit
    }
    fn set_need_exit(&mut self) {
        self.is_need_exit = true;
    }
    pub fn get_winapi(&mut self) -> &mut W {
        &mut self.winapi
    }
}
