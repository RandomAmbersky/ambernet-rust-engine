pub trait TNodeBase {
    type FrameContext;
    fn draw(&mut self, fcx: &mut Self::FrameContext);
}
