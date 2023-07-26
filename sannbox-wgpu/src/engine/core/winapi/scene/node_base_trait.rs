pub trait TNodeBase {
    type FrameContext;
    fn draw(&mut self, gfx: &mut Self::FrameContext);
}
