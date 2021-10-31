//include!(concat!(env!("OUT_DIR"), "/modimports.rs"));
pub use crate::error::Error;

use std::future::Future;
use std::pin::Pin;
use tokio::sync::mpsc::Receiver;

use async_trait::async_trait;

pub type DynModule = Box<dyn Module + Send>;
type DynModuleFuture = Pin<Box<dyn Future<Output = Result<DynModule, Error>>>>;

automod::dir!("src/modules");

pub async fn instantiate_by_name(name: &str, config: toml::Value) -> Result<DynModule, Error> {
    automod::with_mods!("src/modules" PLACEHOLDER => if stringify!(PLACEHOLDER) == name {
        return PLACEHOLDER::instantiate(config).await
    });
    panic!()
}

pub struct Definiton {
    pub name: &'static str,
    pub instantiate: fn(cfg: toml::Value) -> DynModuleFuture,
}

#[async_trait]
pub trait Module {
    fn name(&self) -> String;

    async fn trigger(&mut self, action: &str) -> Option<String>;

    async fn subscribe(&mut self) -> Receiver<(String, String)>;

    fn color(&self) -> (u8, u8, u8) {
        return (230, 100, 20);
    }
}
