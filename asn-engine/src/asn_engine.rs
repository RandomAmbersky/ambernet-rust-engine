use crate::asn_context::AsnContext;

pub fn init() -> AsnContext {
    AsnContext {
        is_need_exit: false,
    }
}

pub fn do_loop(ctx: &AsnContext) {}
