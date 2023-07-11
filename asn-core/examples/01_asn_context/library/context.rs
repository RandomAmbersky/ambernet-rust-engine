use crate::library::WinApiTrait;

pub struct Context<W>
where
    W: WinApiTrait,
{
    winapi: W,
}

impl<W> Context<W>
where
    W: WinApiTrait,
{
    pub fn new(winapi: W) -> Self {
        Self { winapi }
    }
    pub fn get_winapi(&mut self) -> &mut W {
        &mut self.winapi
    }
}
