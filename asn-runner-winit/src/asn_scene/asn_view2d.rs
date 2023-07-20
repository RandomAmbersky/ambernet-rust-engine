use crate::asn_winapi::AsnWgpuWinApi;
use asn_scenegraph_core::AsnScenegraphNodeTrait;
use uuid::Uuid;

pub struct AsnView2d {
    id: Uuid,
}

impl AsnScenegraphNodeTrait<AsnWgpuWinApi> for AsnView2d {
    fn new(api: &AsnWgpuWinApi) -> Self {
        let id = Uuid::new_v4();
        AsnView2d { id }
    }

    fn get_id() -> Uuid {
        todo!()
    }

    fn update() {
        todo!()
    }

    fn draw(api: &AsnWgpuWinApi) {
        todo!()
    }
}
