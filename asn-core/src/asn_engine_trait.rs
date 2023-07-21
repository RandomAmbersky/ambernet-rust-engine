pub trait AsnEngineTrait<'a> {
    type WinApi;
    type Context;
    type Scene;
    fn get_winapi(&mut self) -> &'a mut Self::WinApi;
    fn get_context(&mut self) -> &'a mut Self::Context;
    fn get_scene(&mut self) -> &'a mut Self::Scene;

    fn run(&mut self);
}
