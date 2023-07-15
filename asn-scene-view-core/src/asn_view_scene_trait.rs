use crate::asn_scene_view2d_trait::AsnSceneView2dTrait;
use crate::AsnWinapiTrait;
use uuid::Uuid;

pub trait AsnWinapiSceneTrait {
    type View2d;
    fn delete(id: Uuid);
    fn new_view2d() -> View2d;
}
