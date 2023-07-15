use crate::asn_scene_view2d_trait::AsnSceneView2dTrait;
use uuid::Uuid;

pub trait AsnWinapiSceneTrait {
    type View2d;
    fn delete(id: Uuid);
    fn new_view2d() -> Self::View2d;
}
