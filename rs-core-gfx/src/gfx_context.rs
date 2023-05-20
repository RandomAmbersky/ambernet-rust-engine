use rs_core::AsnError;

pub trait GfxContextTrait<E>
where
    E: AsnError,
{
    fn begin_frame(&mut self) -> Result<(), E>;
    fn end_frame(&mut self) -> Result<(), E>;
}
