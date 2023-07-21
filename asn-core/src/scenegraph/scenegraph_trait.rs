use crate::AsnWinapiTrait;
use uuid::Uuid;
use crate::scenegraph::node_trait::AsnScenegraphNodeTrait;

pub trait AsnScenegraphTrait<W>
where
    W: AsnWinapiTrait,
{
    type View2d: AsnScenegraphNodeTrait<W>;
    fn delete(id: Uuid);
    fn new_view2d(&mut self, api: &W) -> Self::View2d;
}
