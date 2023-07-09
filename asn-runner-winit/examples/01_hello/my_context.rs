use asn_core::AsnContextTrait;
use asn_runner_winit::AsnWgpuWinApi;

pub struct MyCtx {
    is_need_exit: bool,
    winapi: AsnWgpuWinApi,
}

impl AsnContextTrait<AsnWgpuWinApi> for MyCtx {
    fn is_need_exit(&self) -> bool {
        self.is_need_exit
    }

    fn set_need_exit(&mut self) {
        self.is_need_exit = true
    }

    fn winapi(&self) -> &'a AsnWgpuWinApi {
        &self.winapi
    }
}

pub fn new(winapi: AsnWgpuWinApi) -> MyCtx {
    MyCtx {
        winapi,
        is_need_exit: false,
    }
}
