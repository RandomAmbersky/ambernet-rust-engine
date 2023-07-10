use asn_core::{AsnWinapiTrait, Size2D};

struct MyWinApi {}

impl AsnWinapiTrait for MyWinApi {
    fn window_resize(&mut self, size: Size2D<u32>) {
        todo!()
    }
}

type MyContextBuilder<'a> = AsnContextBuilder<'a, MyWinApi>;
type MyContext = AsnContext<'static, MyWinApi>;

fn get_context() -> MyContext {
    let win_api = MyWinApi {};
    let c = MyContextBuilder::new().set_winapi(&win_api).build();
    c
}

fn main() {
    let mut ctx = get_context();
    let check = ctx.is_need_exit();
    ctx.set_need_exit();
    let check2 = ctx.is_need_exit();
}
