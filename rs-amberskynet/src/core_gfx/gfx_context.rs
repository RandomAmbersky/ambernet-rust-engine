use crate::core::AsnError;

pub trait GfxContext<E>
where
    E: AsnError,
{
    fn begin_frame(&mut self) -> Result<(), E>;
    fn end_frame(&mut self) -> Result<(), E>;
}
