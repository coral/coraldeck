use serde::Deserialize;
use std::any::{Any, TypeId};
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

    //    let initfunc:Vec<Box<dyn Fn(cfg: Any)  = Vec::new();

    // initfunc.push(Box::new(MOTU::new());

    for (nm, m) in cfg.modules {
        if nm == "motu" {
            //let v: Result<MOTUConfig, toml::de::Error> = cast_to_config(m);
            //let actual_struct = v.unwrap();
            let am = MOTU::new(m);

            dbg!(am);
        }
    }
}

#[derive(Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct MOTUConfig {
    pub ip: Ipv4Addr,
}

fn cast_to_config<T: serde::de::DeserializeOwned>(esc: toml::Value) -> Result<T, toml::de::Error> {
    esc.try_into()
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct MOTU {
    config: MOTUConfig,
}

impl MOTU {
    pub fn new(m: toml::Value) -> Result<MOTU, toml::de::Error> {
        Ok(MOTU {
            config: m.try_into()?,
        })
    }
}

//Other
#[derive(Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct OtherConfig {
    pub something: String,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Other {
    config: OtherConfig,
}

impl Other {
    pub fn new(m: OtherConfig) -> Other {
        Other { config: m }
    }
}
