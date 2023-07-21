pub trait AsnEngineTrait<'a> {
    type WinApi;
    type Context;
    type Scene;
    fn get_winapi() -> &'a mut Self::WinApi;
    fn get_context() -> &'a mut Self::Context;
    fn get_scene() -> &'a mut Self::Scene;

    fn run(&mut self);
}
