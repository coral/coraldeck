use coraldeck::motu;
use coraldeck::KeyLight;
use std::net::Ipv4Addr;
use std::str::FromStr;
#[tokio::main]
async fn main() {
    // let ip = Ipv4Addr::from_str("10.0.1.5").unwrap();
    // let mut mt = motu::MOTU::new(ip);
    // mt.connect().await.unwrap();

    // mt.set("ext/obank/0/ch/0/stereoTrim", -45).await.unwrap();

    // let myval = mt
    //     .get("ext/obank/0/ch/0/stereoTrim")
    //     .await
    //     .unwrap()
    //     .as_f64()
    //     .unwrap();
    // dbg!(myval);

    KeyLight::new_from_name().await;
    let ip = Ipv4Addr::from_str("10.0.1.32").unwrap();
    let mut kl = KeyLight::new_from_ip(ip);

    let klval = kl.get().await.unwrap();
    dbg!(klval);
}
