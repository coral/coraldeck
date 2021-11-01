pub use crate::error::Error;
use tokio::sync::mpsc::Receiver;

use async_trait::async_trait;

pub type DynModule = Box<dyn Module + Send>;
//type DynModuleFuture = Pin<Box<dyn Future<Output = Result<DynModule, Error>>>>;

automod::dir!("src/modules");

pub async fn instantiate_by_name(name: &str, config: toml::Value) -> Result<DynModule, Error> {
    automod::with_mods!("src/modules" PLACEHOLDER => if stringify!(PLACEHOLDER) == name {
        return PLACEHOLDER::instantiate(config).await
    });

    return Err(Error::ModuleNotFound(name.to_string()));
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
