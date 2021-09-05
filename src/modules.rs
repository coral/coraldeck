pub mod motu;

pub use motu::MOTU;

use async_trait::async_trait;

#[async_trait]
pub trait Module {
    fn name(&self) -> String;

    async fn trigger(&mut self, action: &str) -> Option<String>;
}
