//include!(concat!(env!("OUT_DIR"), "/modimports.rs"));

pub use crate::error::Error;
use tokio::sync::mpsc::Receiver;

use async_trait::async_trait;

inventory::collect!(Box<dyn Module + Send>);

#[async_trait]
pub trait Module {
    fn name(&self) -> String;

    async fn trigger(&mut self, action: &str) -> Option<String>;

    async fn subscribe(&mut self) -> Receiver<(String, String)>;
}
