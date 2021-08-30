use log::error;
use rand::Rng;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::fmt::Display;
use std::net::Ipv4Addr;
use std::ops::DerefMut;
use std::sync::Arc;
use std::time::Duration;
use thiserror::Error;
use tokio::sync::oneshot;
use tokio::sync::Mutex;
use tokio::time;

#[derive(Error, Debug)]
pub enum MOTUError {
    #[error("ParseError")]
    ParseError,

    #[error(transparent)]
    RequestError(#[from] reqwest::Error),
}

#[allow(dead_code)]
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
        let data =
            MOTU::get_cache(self.client.clone(), self.url.clone(), self.cache.clone()).await?;

        let cacheref = self.cache.clone();
        *cacheref.lock().await.deref_mut() = data;

        let cacheref = self.cache.clone();
        let url = self.url.clone();
        let client = self.client.clone();
        tokio::spawn(async move {
            let mut interval = time::interval(Duration::from_secs(10));
            loop {
                interval.tick().await;

                match MOTU::get_cache(client.clone(), url.clone(), cacheref.clone()).await {
                    Ok(data) => {
                        *cacheref.lock().await.deref_mut() = data;
                    }

                    Err(_) => {}
                }
            }
        });

        Ok(())
    }

    async fn get_cache(
        client: reqwest::Client,
        url: String,
        cacheref: Arc<Mutex<HashMap<String, Value>>>,
    ) -> Result<HashMap<String, Value>, MOTUError> {
        let resp = client.get(&url).send().await?;

        Ok(resp.json::<HashMap<String, Value>>().await?)
    }

    pub async fn get(&self, key: &str) -> Option<Value> {
        match self.cache.lock().await.get(&key.to_string()) {
            Some(v) => Some(v.clone()),
            None => None,
        }
    }

    pub async fn set<T: Display + serde::ser::Serialize>(
        &mut self,
        key: &str,
        value: T,
    ) -> Result<reqwest::Response, MOTUError> {
        let url = self.url.clone();

        let form =
            reqwest::multipart::Form::new().text("json", format!("{{\"{}\": {}}}", key, value));

        let res = self.client.patch(url).multipart(form).send().await?;

        let ok = self
            .cache
            .lock()
            .await
            .insert(key.to_string(), json!(value));

        Ok(res)
    }
}
