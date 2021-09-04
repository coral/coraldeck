#[async_trait]
pub trait Action {
    fn trigger(&mut self);
}
