use crate::AsnWinapiTrait;

pub trait AsnContextTrait<'a, W>
where
    W: AsnWinapiTrait,
{
    fn is_need_exit(&self) -> bool;
    fn set_need_exit(&mut self);
    fn winapi(&self) -> &'a W;
}

pub struct AsnContext<'a, W>
where
    W: AsnWinapiTrait,
{
    is_need_exit: bool,
    winapi: &'a W,
}

impl<'a, W> AsnContextTrait<'a, W> for AsnContext<'a, W>
where
    W: AsnWinapiTrait,
{
    fn is_need_exit(&self) -> bool {
        self.is_need_exit
    }

    fn set_need_exit(&mut self) {
        self.is_need_exit = true
    }

    fn winapi(&self) -> &'a W {
        self.winapi
    }
}
