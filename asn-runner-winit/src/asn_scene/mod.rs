use crate::asn_scene::asn_view2d::AsnView2d;
use crate::asn_winapi::AsnWgpuWinApi;
use asn_scenegraph_core::{AsnScenegraphNodeTrait, AsnScenegraphTrait};

pub mod asn_view2d;

pub struct AsnWgpuScene {}

impl AsnWgpuScene {
    pub fn new() -> Self {
        AsnWgpuScene {}
    }
}

impl AsnScenegraphTrait<AsnWgpuWinApi> for AsnWgpuScene {
    type View2d = AsnView2d;

    fn delete(_id: uuid::Uuid) {
        todo!()
    }

    fn new_view2d(&mut self, api: &AsnWgpuWinApi) -> Self::View2d {
        AsnView2d::new(api)
    }
}
