use coraldeck::motu;
use coraldeck::KeyLight;
use std::net::Ipv4Addr;
use std::str::FromStr;
use std::time::Duration;
#[tokio::main]
async fn main() {
    let mut kl = KeyLight::new_from_name("Key Light Left").await;

    dbg!(kl);

    // let klval = kl.get().await.unwrap();
    // dbg!(klval);

    tokio::time::sleep(Duration::from_secs(5)).await;
}
