use crate::node_trait::AsnScenegraphNodeTrait;
use asn_core::AsnWinapiTrait;
use uuid::Uuid;

pub trait AsnScenegraphTrait<W>
where
    W: AsnWinapiTrait,
{
    type View2d: AsnScenegraphNodeTrait<W>;
    fn delete(id: Uuid);
    fn new_view2d(&mut self, api: &W) -> Self::View2d;
}
