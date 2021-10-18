include!(concat!(env!("OUT_DIR"), "/modimports.rs"));
pub use crate::error::Error;
use std::future::Future;
use tokio::sync::mpsc::Receiver;

use async_trait::async_trait;

pub struct Definiton {
    pub name: &'static str,
    //Box<dyn Future<Output = Result<Box<dyn Module + Send>, Error>>>
    //pub instansiate: Box<dyn Fn() -> Result<Box<dyn Module + Send>, Error>>,
    pub instansiate:
        Box<dyn Fn() -> Box<dyn Future<Output = Result<Box<dyn Module + Send>, Error>>>>,
}

inventory::collect!(Definiton);

#[async_trait]
pub trait Module {
    fn name(&self) -> String;

    async fn trigger(&mut self, action: &str) -> Option<String>;

    async fn subscribe(&mut self) -> Receiver<(String, String)>;
}
