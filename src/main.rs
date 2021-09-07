mod config;
mod controller;
mod graphics;
mod modules;
mod sman;

use config::Config;
use controller::Controller;
use modules::{Camera, KeyLight, KeyLights, Module, MOTU};
use sman::StreamDeckManager;
use std::time::Duration;

#[macro_use]
extern crate lazy_static;

#[tokio::main]
async fn main() {
    let cfg = Config::load_config("files/config.json").unwrap();

    let mut sman = StreamDeckManager::new().await.unwrap();
    let mut sgfx = graphics::Startup::new(&mut sman).await;

    // Module init

    let mut m: Vec<Box<dyn Module + Send>> = Vec::new();
    {
        //Motu
        let mut motu = MOTU::new(cfg.devices.motu);
        motu.connect().await.unwrap();
        m.push(Box::new(motu));
        sgfx.load(&mut sman, "MOTU").await;

        //Keylights
        let mut lights: Vec<KeyLight> = Vec::new();
        for l in &cfg.devices.keylight {
            let mut key = KeyLight::new_from_name(&l, true).await.unwrap();
            lights.push(key);
        }
        let kl = KeyLights::new(lights).await;
        m.push(Box::new(kl));
        sgfx.load(&mut sman, "KEYLIGHT").await;

        //Camera
        let mut cam = Camera::new(&cfg.devices.camera).await.unwrap();
        cam.connect(Duration::from_secs(10)).await.unwrap();
        m.push(Box::new(cam));
        sgfx.load(&mut sman, "CAMERA").await;
    }
    // Controller

    let mut ctrl = Controller::new(cfg, sman, m).await;

    let mut handles = vec![];

    handles.push(tokio::spawn(async move {
        ctrl.spin().await;
    }));

    futures::future::join_all(handles).await;
}
