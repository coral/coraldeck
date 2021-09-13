pub mod camera;
pub mod keylight;
pub mod motu;

pub use camera::Camera;
pub use keylight::{KeyLight, KeyLights};
pub use motu::MOTU;
use tokio::sync::mpsc::Receiver;

use async_trait::async_trait;

#[async_trait]
pub trait Module {
    fn name(&self) -> String;

    async fn trigger(&mut self, action: &str) -> Option<String>;

    async fn subscribe(&mut self) -> Receiver<(String, String)>;
}
