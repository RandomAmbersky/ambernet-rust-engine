pub trait AsnRunnerTrait {
    type AsnContext;
    type AsnHandler;
    fn run(self, ctx: Self::AsnContext, h: Self::AsnHandler);
}
