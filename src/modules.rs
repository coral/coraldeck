//include!(concat!(env!("OUT_DIR"), "/modimports.rs"));
pub use crate::error::Error;
use for_each_mod::*;

use std::future::Future;
use std::pin::Pin;
use tokio::sync::mpsc::Receiver;

use async_trait::async_trait;

type DynModule = Box<dyn Module + Send>;
type DynModuleFuture = Pin<Box<dyn Future<Output = Result<DynModule, Error>>>>;

for_each_mod! {
    #[path = "modules/PLACEHOLDER.rs"]
    mod PLACEHOLDER;
}

pub async fn instantiate_by_name(name: &str) -> Result<DynModule, Error> {
    for_each_mod! { if "PLACEHOLDER" == name { return PLACEHOLDER::instantiate().await; } };
    panic!()
}

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
