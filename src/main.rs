mod config;
mod controller;
mod graphics;
mod modules;
mod sman;

use config::Config;
use controller::{Controller, ModuleConfig};
use modules::{Camera, KeyLight, KeyLights, Module, MOTU};
use sman::StreamDeckManager;
use std::time::Duration;

#[macro_use]
extern crate lazy_static;

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    info!("Starting CORALDECK");
    let cfg = Config::load_config("files/config.json").unwrap();

    let mut sman = StreamDeckManager::new().await.unwrap();
    let mut sgfx = graphics::Startup::new(&mut sman).await;

    // Module init

    let mut m: Vec<ModuleConfig> = Vec::new();
    {
        //Motu
        let mut motu = MOTU::new(cfg.devices.motu.ip);
        motu.connect().await.unwrap();
        m.push(ModuleConfig {
            module: Box::new(motu),
            color: cfg.devices.motu.color,
        });
        sgfx.load(&mut sman, "MOTU").await;

        //Keylights
        let mut lights: Vec<KeyLight> = Vec::new();
        for l in &cfg.devices.keylight.names {
            let mut key = KeyLight::new_from_name(&l, true).await.unwrap();
            lights.push(key);
        }
        let kl = KeyLights::new(lights).await;
        m.push(ModuleConfig {
            module: Box::new(kl),
            color: cfg.devices.keylight.color,
        });
        sgfx.load(&mut sman, "KEYLIGHT").await;

        //Camera
        let mut cam = Camera::new(&cfg.devices.camera.name).await.unwrap();
        m.push(ModuleConfig {
            module: Box::new(cam),
            color: cfg.devices.camera.color,
        });
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
