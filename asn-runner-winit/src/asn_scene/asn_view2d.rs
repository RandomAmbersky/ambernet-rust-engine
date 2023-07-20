use crate::asn_winapi::AsnWgpuWinApi;
use asn_scene_view_core::AsnSceneViewNodeTrait;
use uuid::Uuid;

pub struct AsnView2d {
    id: Uuid,
}

impl AsnSceneViewNodeTrait<AsnWgpuWinApi> for AsnView2d {
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
