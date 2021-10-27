include!(concat!(env!("OUT_DIR"), "/modimports.rs"));
pub use crate::error::Error;
use std::future::Future;
use std::pin::Pin;
use tokio::sync::mpsc::Receiver;

use async_trait::async_trait;

type DynModule = Box<dyn Module + Send>;
type DynModuleFuture = Pin<Box<dyn Future<Output = Result<DynModule, Error>>>>;

pub struct Definiton {
    pub name: &'static str,
    pub instantiate: fn() -> DynModuleFuture,
}

#[async_trait]
pub trait Module {
    fn name(&self) -> String;

    async fn trigger(&mut self, action: &str) -> Option<String>;

    async fn subscribe(&mut self) -> Receiver<(String, String)>;
}
