mod config;
mod controller;
mod error;
mod graphics;
mod modules;
mod sman;

use config::Config;
use controller::Controller;
use sman::StreamDeckManager;
use std::time::Duration;

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

#[tokio::main]
async fn main() -> Result<(), error::Error> {
    pretty_env_logger::init();
    info!("Starting CORALDECK");

    let cfg = Config::load_config("files/config.toml").unwrap();

    let mut sman = StreamDeckManager::new().await.unwrap();
    let mut loaded_modules: Vec<modules::DynModule> = Vec::new();

    //Loading sequence
    {
        let mut boot = graphics::Boot::new(&mut sman);
        boot.header().await;

        for (name, module) in cfg.modules.clone() {
            let imod = modules::instantiate_by_name(&name, module).await?;
            boot.load(&imod.name().to_uppercase()).await;
            loaded_modules.push(imod);
        }
    }

    sman.reset().await;

    let mut ctrl = Controller::new(cfg.clone(), sman, loaded_modules).await;

    let mut handles = vec![];

    handles.push(tokio::spawn(async move {
        ctrl.spin().await;
    }));

    futures::future::join_all(handles).await;

    Ok(())
}
