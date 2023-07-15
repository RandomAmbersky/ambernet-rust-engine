use crate::asn_winapi::asn_view2d::AsnView2d;
use crate::asn_winapi::AsnWgpuWinApi;
use asn_scene_view_core::AsnSceneViewTrait;

pub struct AsnScene {}

impl AsnScene {
    pub fn new() -> Self {
        AsnScene {}
    }
}

impl AsnSceneViewTrait<AsnWgpuWinApi> for AsnScene {
    type View2d = AsnView2d;

    fn delete(id: uuid::Uuid) {
        todo!()
    }

    fn new_view2d() -> AsnView2d {
        AsnView2d::new()
    }
}
