use serde::Deserialize;
use std::any::Any;
use std::net::Ipv4Addr;
use std::{collections::BTreeMap, fs};

#[derive(Deserialize, Debug)]
struct Config {
    modules: BTreeMap<String, toml::Value>,
}

#[derive(Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Module {
    name: String,
}

fn main() {
    let data = fs::read_to_string("files/config.toml").unwrap();
    let cfg: Config = toml::from_str(&data).unwrap();

    dbg!(&cfg);

    for (_, m) in cfg.modules {
        match cast_to_config(m) {
            Ok(v) => {
                let x: Box<dyn Any> = Box::new(v);

                dbg!(x);
            }
            Err(_) => {}
        }
    }
}

#[derive(Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct MOTU {
    pub ip: Ipv4Addr,
}

fn cast_to_config(esc: toml::Value) -> Result<MOTU, toml::de::Error> {
    esc.try_into()
}
