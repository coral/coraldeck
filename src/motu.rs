use log::error;
use rand::Rng;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;
use std::net::{IpAddr, Ipv4Addr};
use std::ops::DerefMut;
use std::sync::Arc;
use std::time::Duration;
use thiserror::Error;
use tokio::sync::Mutex;
use tokio::time;

#[derive(Error, Debug)]
pub enum MOTUError {
    #[error("ParseError")]
    ParseError,

    #[error(transparent)]
    RequestError(#[from] reqwest::Error),
}

pub struct MOTU {
    addr: Ipv4Addr,
    client_id: u32,
    url: String,

    client: reqwest::Client,
    cache: Arc<Mutex<HashMap<String, Value>>>,
}

impl MOTU {
    pub fn new(addr: Ipv4Addr) -> MOTU {
        let mut rng = rand::thread_rng();
        let client_id = rng.gen::<u32>();

        let url = format!("http://{}/datastore?client={}", addr.to_string(), client_id);

        MOTU {
            addr,
            client_id,
            url,

            client: reqwest::Client::new(),
            cache: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn connect(&mut self) -> Result<(), MOTUError> {
        let cacheref = self.cache.clone();
        let url = self.url.clone();
        let client = self.client.clone();

        tokio::spawn(async move {
            let mut interval = time::interval(Duration::from_secs(10));
            loop {
                interval.tick().await;

                let resp = match client.get(&url).send().await {
                    Ok(v) => v,
                    Err(e) => {
                        error!("{}", e);
                        continue;
                    }
                };

                match resp.json::<HashMap<String, Value>>().await {
                    Ok(data) => {
                        *cacheref.lock().await.deref_mut() = data;
                    }
                    Err(_) => {}
                }
            }
        });

        Ok(())
    }

    pub async fn set<T: Display + serde::ser::Serialize>(
        &mut self,
        key: &str,
        value: T,
    ) -> Result<reqwest::Response, MOTUError> {
        let url = self.url.clone();

        dbg!(format!("{{\"{}\": {}}}", key, value));

        let form =
            reqwest::multipart::Form::new().text("json", format!("{{\"{}\": {}}}", key, value));

        let res = self.client.patch(url).multipart(form).send().await?;

        self.cache
            .lock()
            .await
            .insert(key.to_string(), json!(value));

        Ok(res)
    }
}
