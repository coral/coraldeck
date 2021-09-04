mod config;
mod render;
mod sman;

use config::Config;
use sman::StreamDeckManager;

#[tokio::main]
async fn main() {
    let cfg = Config::load_config("files/config.json").unwrap();
    dbg!(cfg);

    let sman = StreamDeckManager::new().await.unwrap();
}
