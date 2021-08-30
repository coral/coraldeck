use coraldeck::motu;
use std::net::{IpAddr, Ipv4Addr};
use std::str::FromStr;
use std::time::Duration;
#[tokio::main]
async fn main() {
    let ip = Ipv4Addr::from_str("10.0.1.5").unwrap();
    let mut mt = motu::MOTU::new(ip);
    mt.connect().await.unwrap();

    mt.set("ext/obank/0/ch/0/stereoTrim", -45).await.unwrap();

    let myval = mt
        .get("ext/obank/0/ch/0/stereoTrim")
        .await
        .unwrap()
        .as_f64()
        .unwrap();
    dbg!(myval);
}
