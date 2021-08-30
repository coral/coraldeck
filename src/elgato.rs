use serde::{Deserialize, Serialize};
use std::net::Ipv4Addr;
use thiserror::Error;
use zeroconf::prelude::*;
use zeroconf::{MdnsBrowser, ServiceDiscovery, ServiceType};
use tokio::sync::mpsc;
use std::any::Any;
use std::sync::Arc;

#[derive(Error, Debug)]
pub enum ElgatoError {
    #[error("ParseError")]
    ParseError,

    #[error(transparent)]
    RequestError(#[from] reqwest::Error),
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Status {
    pub number_of_lights: i64,
    pub lights: Vec<Light>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Light {
    pub on: i64,
    pub brightness: i64,
    pub temperature: i64,
}

#[allow(dead_code)]
pub struct KeyLight {
    addr: Ipv4Addr,
    url: String,

    client: reqwest::Client,
}

impl KeyLight {
    pub fn new_from_ip(addr: Ipv4Addr) -> KeyLight {
        KeyLight {
            addr,
            url: format!("http://{}:9123/elgato/lights", addr.to_string()),

            client: reqwest::Client::new(),
        }
    }

    pub async fn new_from_name() {
        let mut browser = MdnsBrowser::new(ServiceType::new("elg", "tcp").unwrap());

        let (tx, rx) = mpsc::channel(16);

        browser.set_service_discovered_callback( Box::new(move |result: zeroconf::Result<ServiceDiscovery>,
            _context: Option<Arc<dyn Any>>| {

            let res = result.unwrap();
            tx.send(res);
        }));


    
        let event_loop = browser.browse_services().unwrap()
    }
    

    pub async fn get(&mut self) -> Result<Status, ElgatoError> {
        let resp = self.client.get(&self.url).send().await?;

        Ok(resp.json::<Status>().await?)
    }
}
