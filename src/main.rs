mod config;
mod controller;
mod graphics;
mod modules;
mod sman;

use config::Config;
use controller::Controller;
use modules::{Module, MOTU};
use sman::StreamDeckManager;

#[macro_use]
extern crate lazy_static;

#[tokio::main]
async fn main() {
    let cfg = Config::load_config("files/config.json").unwrap();

    // Module init

    let mut m: Vec<Box<dyn Module + Send>> = Vec::new();
    {
        //Motu
        let mut motu = MOTU::new(cfg.devices.motu);
        motu.connect().await.unwrap();
        m.push(Box::new(motu));
    }
    // Controller

    let sman = StreamDeckManager::new().await.unwrap();

    let mut ctrl = Controller::new(cfg, sman, m).await;

    let mut handles = vec![];

    handles.push(tokio::spawn(async move {
        ctrl.spin().await;
    }));

    futures::future::join_all(handles).await;
}
