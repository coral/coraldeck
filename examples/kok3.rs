use coraldeck::sman::StreamDeckManager;
use coraldeck::Config;
use std::time::Duration;

#[tokio::main]
async fn main() {
    let m = Config::load_config("files/config.json").unwrap();

    let mut sman = StreamDeckManager::new().await.unwrap();

    let mut c = sman.subscribe().await;

    loop {
        let val = c.recv().await;
        dbg!(val);
    }

    tokio::time::sleep(Duration::from_secs(10)).await;
}
