use crate::asn_scene_view_node_trait::AsnSceneViewNodeTrait;
use asn_core::AsnWinapiTrait;
use uuid::Uuid;

pub trait AsnSceneViewTrait<W>
where
    W: AsnWinapiTrait,
{
    type View2d: AsnSceneViewNodeTrait<W>;
    fn delete(id: Uuid);
    fn new_view2d(&mut self, api: &W) -> Self::View2d;
}
