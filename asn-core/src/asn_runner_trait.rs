use crate::AsnContext;

pub trait AsnRunnerTrait {
    type AsnHandler;
    fn run(self, ctx: &mut AsnContext);
}
