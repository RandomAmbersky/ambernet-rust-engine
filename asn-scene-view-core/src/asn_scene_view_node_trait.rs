use asn_core::AsnWinapiTrait;

pub trait AsnSceneViewNodeTrait<W>
where
    W: AsnWinapiTrait,
{
    fn update();
    fn draw(api: &W);
}
