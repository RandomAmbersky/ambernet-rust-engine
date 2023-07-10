use crate::AsnWinapiTrait;

pub trait AsnContextTrait<'a, W>
where
    W: AsnWinapiTrait,
{
    fn is_need_exit(&self) -> bool;
    fn set_need_exit(&mut self);
    fn winapi(&self) -> &'a W;
}
