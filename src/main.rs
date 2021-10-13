mod config;
mod controller;
mod error;
mod graphics;
mod modules;
mod sman;

use config::Config;
use controller::{Controller, ModuleConfig};
use modules::{Camera, KeyLight, KeyLights, MOTU};
use sman::StreamDeckManager;
use std::time::Duration;

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    info!("Starting CORALDECK");
    let cfg = Config::load_config("files/config.json").unwrap();

    let mut sman = StreamDeckManager::new().await.unwrap();

    // Module init
    let mut m: Vec<ModuleConfig> = Vec::new();
    {
        let mut boot = graphics::Boot::new(&mut sman);
        boot.header().await;

        //Motu
        let mut motu = MOTU::new(cfg.devices.motu.ip);
        motu.connect().await.unwrap();
        m.push(ModuleConfig {
            module: Box::new(motu),
            color: cfg.devices.motu.color,
        });
        boot.load("MOTU").await;

        //Keylights
        let mut lights: Vec<KeyLight> = Vec::new();
        for l in &cfg.devices.keylight.names {
            let key = KeyLight::new_from_name(&l, Some(Duration::from_secs(5)))
                .await
                .unwrap();
            lights.push(key);
        }
        let kl = KeyLights::new(lights).await;
        m.push(ModuleConfig {
            module: Box::new(kl),
            color: cfg.devices.keylight.color,
        });
        boot.load("KEYLIGHT").await;

        //Camera
        let cam = Camera::new(&cfg.devices.camera.name).await.unwrap();
        m.push(ModuleConfig {
            module: Box::new(cam),
            color: cfg.devices.camera.color,
        });
        boot.load("CAMERA").await;
    }
    // Controller

    let mut ctrl = Controller::new(cfg, sman, m).await;

    let mut handles = vec![];

    handles.push(tokio::spawn(async move {
        ctrl.spin().await;
    }));

    futures::future::join_all(handles).await;
}
