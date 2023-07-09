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

pub struct AsnContextBuilder<'a, W>
where
    W: AsnWinapiTrait,
{
    winapi: Option<&'a W>,
}

impl<'a, W> AsnContextBuilder<'a, W>
where
    W: AsnWinapiTrait,
{
    pub fn new() -> AsnContextBuilder<'a, W> {
        AsnContextBuilder { winapi: None }
    }
    pub fn build(&self) -> AsnContext<'a, W> {
        AsnContext {
            is_need_exit: false,
            winapi: self.winapi.unwrap(),
        }
    }
}
