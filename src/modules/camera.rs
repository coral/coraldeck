use crate::modules::Module;
use async_trait::async_trait;
use big_s::S;
use blackmagic_camera_control::command::{Command, Video};
pub use blackmagic_camera_control::BluetoothCamera;
use blackmagic_camera_control::Operation;
use serde::Deserialize;

use crate::error::Error;
use lazy_static::lazy_static;
use std::time::Duration;
use tokio::sync::broadcast::error::RecvError;
use tokio::sync::mpsc::{self, Receiver};

lazy_static! {
    static ref ISO: Vec<i32> = vec![
        100, 125, 160, 200, 250, 320, 400, 500, 640, 800, 1000, 1250, 1600, 2000, 2500, 3200, 4000,
        5000, 6400, 8000, 10000, 12800, 16000, 20000, 25600,
    ];
}

#[derive(Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct CameraConfig {
    pub name: String,
    pub color: Vec<u8>,
}

#[allow(dead_code)]
pub struct Camera {
    config: CameraConfig,
    cam: BluetoothCamera,
}

impl Camera {
    pub async fn instantiate(cfg: CameraConfig) -> Result<Camera, Error> {
        let mut cam = BluetoothCamera::new(&cfg.name)
            .await
            .map_err(|x| Error::ModuleInit("bmc".to_string(), x.to_string()))?;
        cam.connect(Duration::from_secs(10))
            .await
            .map_err(|x| Error::ModuleInit("bmc".to_string(), x.to_string()))?;

        Ok(Camera { config: cfg, cam })
    }
}

#[async_trait]
impl Module for Camera {
    fn name(&self) -> String {
        return S("camera");
    }

    async fn trigger(&mut self, action: &str) -> Option<String> {
        match action {
            "iso_up" => iso(&mut self.cam, "up").await,
            "iso_down" => iso(&mut self.cam, "down").await,
            "wb_up" => wb(&mut self.cam, 200).await,
            "wb_down" => wb(&mut self.cam, -200).await,
            _ => None,
        }
    }

    async fn subscribe(&mut self) -> Receiver<(String, String)> {
        let (tx, rx) = mpsc::channel(16);
        let mut camupdates = self.cam.updates().await;
        let name = self.name();

        tokio::spawn(async move {
            loop {
                let update: Result<Command, RecvError> = camupdates.recv().await;
                match update {
                    Ok(u) => {
                        let _ = tx
                            .send((
                                format!(
                                    "{}_{}_{}",
                                    &name,
                                    u.normalized_name().0,
                                    u.normalized_name().1
                                ),
                                u.to_string(),
                            ))
                            .await;
                    }
                    Err(_) => {}
                }
            }
        });

        rx
    }
}

async fn iso(cam: &mut BluetoothCamera, direction: &str) -> Option<String> {
    match cam.get_normalized("video_iso").await {
        Some(current_value) => {
            if let Command::Video(Video::Iso(iso)) = current_value {
                match ISO.iter().position(|&r| r == iso) {
                    Some(i) => {
                        let nv = match direction {
                            "up" => {
                                if i < ISO.len() {
                                    ISO[i + 1]
                                } else {
                                    ISO[i]
                                }
                            }
                            "down" => {
                                if i > 0 {
                                    ISO[i - 1]
                                } else {
                                    ISO[i]
                                }
                            }
                            _ => 0,
                        };

                        let _ = cam
                            .write(255, Operation::AssignValue, Command::Video(Video::Iso(nv)))
                            .await;

                        return None;
                    }
                    None => None,
                }
            } else {
                None
            }
        }
        None => None,
    }
}

async fn wb(cam: &mut BluetoothCamera, diff: i16) -> Option<String> {
    match cam.get_normalized("video_manual_white_balance").await {
        Some(current_value) => {
            if let Command::Video(Video::ManualWhiteBalance(wbdata)) = current_value {
                let nv = wbdata[0] + diff;

                let _ = cam
                    .write(
                        255,
                        Operation::AssignValue,
                        Command::Video(Video::ManualWhiteBalance(vec![nv, wbdata[1]])),
                    )
                    .await;

                return Some(format!("{} K", nv.to_string()));
            } else {
                None
            }
        }
        None => None,
    }
}

pub async fn instantiate(cfg: toml::Value) -> Result<super::DynModule, super::Error> {
    let config: CameraConfig = cfg.try_into()?;

    Ok(Box::new(Camera::instantiate(config).await?))
}
