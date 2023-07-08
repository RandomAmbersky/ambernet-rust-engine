use asn_core::AsnContextTrait;

pub struct AsnContext {}

pub fn init() -> AsnContext {
    AsnContext {
        is_need_exit: false,
    }
}

pub fn do_loop(ctx: &AsnContext) {}
