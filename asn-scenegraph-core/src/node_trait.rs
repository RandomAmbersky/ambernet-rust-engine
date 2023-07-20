use asn_core::AsnWinapiTrait;

pub trait AsnScenegraphNodeTrait<W>
where
    W: AsnWinapiTrait,
{
    fn new(api: &W) -> Self;
    fn get_id() -> uuid::Uuid;
    fn update();
    fn draw(api: &W);
}
