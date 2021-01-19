pub type SystemType = uuid::Uuid;
pub type SystemBox = Box<dyn SystemApi>;

pub trait SystemApi {
    fn process(&self);
}
