use crate::{AsnError, AsnEvent};

pub trait AsnHandlerTrait {
    fn proceed(evt: &AsnEvent) -> AsnError;
}
