use crate::AsnWinapiTrait;

pub struct AsnContext<W>
where
    W: AsnWinapiTrait,
{
    winapi: W,
}

impl<W> AsnContext<W>
where
    W: AsnWinapiTrait,
{
    pub fn new(winapi: W) -> Self {
        Self { winapi }
    }
    pub fn get_winapi(&mut self) -> &mut W {
        &mut self.winapi
    }
}
