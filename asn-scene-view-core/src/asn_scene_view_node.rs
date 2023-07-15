use asn_core::AsnWinapiTrait;

pub trait AsnWinapiSceneNode<W>
where
    W: AsnWinapiTrait,
{
    fn update();
    fn draw(api: &W);
}
