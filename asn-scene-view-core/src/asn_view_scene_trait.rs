use crate::AsnWinapiTrait;
use uuid::Uuid;

pub trait AsnWinapiSceneTrait {
    fn delete(id: Uuid);
}

pub trait AsnWinapiSceneNode<W>
where
    W: AsnWinapiTrait,
{
    fn update();
    fn draw(api: &W);
}
