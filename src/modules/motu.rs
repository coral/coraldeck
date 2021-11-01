use crate::error::Error;
use crate::modules::Module;
use async_trait::async_trait;
use big_s::S;
use rand::Rng;
use serde::Deserialize;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::fmt::Display;
use std::net::Ipv4Addr;
use std::ops::DerefMut;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc::Sender;
use tokio::sync::Mutex;
use tokio::time;

#[derive(Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct MOTUConfig {
    pub ip: Ipv4Addr,
    pub color: Vec<u8>,
}

#[allow(dead_code)]
pub struct MOTU {
    config: MOTUConfig,
    client_id: u32,
    url: String,

    client: reqwest::Client,
    cache: Arc<Mutex<HashMap<String, Value>>>,

    last_headphone_vol: f64,
    rendtrig: Option<Sender<(String, String)>>,
}

impl MOTU {
    pub async fn new(cfg: MOTUConfig) -> Result<MOTU, Error> {
        let mut rng = rand::thread_rng();
        let client_id = rng.gen::<u32>();

        //TODO: Resolve difference with client_id
        let url = format!("http://{}/datastore", cfg.ip.to_string());

        let mut m = MOTU {
            config: cfg,
            client_id,
            url,

            client: reqwest::Client::new(),
            cache: Arc::new(Mutex::new(HashMap::new())),

            last_headphone_vol: 0.0,
            rendtrig: None,
        };

        m.connect().await?;

        Ok(m)
    }

    pub async fn connect(&mut self) -> Result<(), Error> {
        let data = MOTU::get_cache(self.client.clone(), self.url.clone()).await?;

        *self.cache.clone().lock().await.deref_mut() = data;

        let cacheref = self.cache.clone();
        let url = self.url.clone();
        let client = self.client.clone();
        tokio::spawn(async move {
            let mut interval = time::interval(Duration::from_secs(5));
            loop {
                match MOTU::get_cache(client.clone(), url.clone()).await {
                    Ok(data) => {
                        *cacheref.lock().await.deref_mut() = data;
                    }

                    Err(_) => {}
                }

                interval.tick().await;
            }
        });

        Ok(())
    }

    async fn get_cache(
        client: reqwest::Client,
        url: String,
    ) -> Result<HashMap<String, Value>, Error> {
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
    ) -> Result<reqwest::Response, Error> {
        let url = self.url.clone();

        let form =
            reqwest::multipart::Form::new().text("json", format!("{{\"{}\": {}}}", key, value));

        let res = self.client.patch(url).multipart(form).send().await?;

        let _ = self
            .cache
            .lock()
            .await
            .insert(key.to_string(), json!(value));

        Ok(res)
    }

    pub async fn set_relative(&mut self, key: &str, value: f64) -> Option<String> {
        let current = match self.get(key).await {
            Some(v) => match v.as_f64() {
                Some(f) => f,
                None => {
                    return None;
                }
            },
            None => {
                return None;
            }
        };
        let new_value = current + value;

        match self.set(key, new_value).await {
            Ok(_) => Some(format!("{} dB", new_value.to_string())),
            Err(_) => None,
        }
    }

    pub async fn mute(&mut self, key: &str) -> Option<String> {
        let current = match self.get(key).await {
            Some(v) => match v.as_f64() {
                Some(f) => f,
                None => {
                    return None;
                }
            },
            None => {
                return None;
            }
        };

        if current == -127.0 {
            self.set(key, self.last_headphone_vol).await.ok();
            Some(format!("{} dB", self.last_headphone_vol))
        } else {
            self.last_headphone_vol = current;
            self.set(key, -127.0).await.ok();

            Some(format!("MUTED"))
        }
    }
}

#[async_trait]
impl Module for MOTU {
    fn name(&self) -> String {
        return S("motu");
    }

    async fn trigger(&mut self, action: &str) -> Option<String> {
        match action {
            "vol_up" => self.set_relative("ext/obank/0/ch/0/stereoTrim", 5.0).await,
            "vol_down" => self.set_relative("ext/obank/0/ch/0/stereoTrim", -5.0).await,
            "mute" => self.mute("ext/obank/0/ch/0/stereoTrim").await,
            _ => None,
        }
    }

    async fn subscribe(&mut self) -> tokio::sync::mpsc::Receiver<(String, String)> {
        let (tx, rx) = tokio::sync::mpsc::channel(16);

        self.rendtrig = Some(tx.clone());

        let cache = self.cache.clone();
        tokio::spawn(async move {
            match cache.lock().await.get("ext/obank/0/ch/0/stereoTrim") {
                Some(val) => {
                    let _ = tx
                        .send(("motu_volume".to_string(), format!("{} dB", val.to_string())))
                        .await;
                }
                None => {}
            };
        });

        rx
    }

    fn color(&self) -> (u8, u8, u8) {
        return (
            self.config.color[0],
            self.config.color[1],
            self.config.color[2],
        );
    }
}

pub async fn instantiate(cfg: toml::Value) -> Result<super::DynModule, super::Error> {
    let config: MOTUConfig = cfg.try_into()?;

    Ok(Box::new(MOTU::new(config).await?))
}
