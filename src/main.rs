mod config;
mod controller;
mod render;
mod sman;

use config::Config;
use controller::Controller;
use sman::StreamDeckManager;

#[macro_use]
extern crate lazy_static;

#[tokio::main]
async fn main() {
    let cfg = Config::load_config("files/config.json").unwrap();

    let sman = StreamDeckManager::new().await.unwrap();

    let mut ctrl = Controller::new(cfg).await;

    let mut handles = vec![];

    handles.push(tokio::spawn(async move {
        ctrl.spin().await;
    }));

    futures::future::join_all(handles).await;
}
