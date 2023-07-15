use crate::asn_winapi::AsnWgpuWinApi;
use asn_scene_view_core::AsnSceneViewNodeTrait;
use uuid::Uuid;

pub struct AsnView2d {}

impl AsnView2d {
    pub fn new() -> Self {
        AsnView2d {}
    }
}

impl AsnSceneViewNodeTrait<AsnWgpuWinApi> for AsnView2d {
    fn update() {
        todo!()
    }

    fn draw(api: &AsnWgpuWinApi) {
        todo!()
    }
}
