use crate::modules::Module;
use async_trait::async_trait;
use big_s::S;
use blackmagic_camera_control::command::{Command, Video};
pub use blackmagic_camera_control::BluetoothCamera;
use blackmagic_camera_control::Operation;

use lazy_static::lazy_static;

lazy_static! {
    static ref ISO: Vec<i32> = vec![
        100, 125, 160, 200, 250, 320, 400, 500, 640, 800, 1000, 1250, 1600, 2000, 2500, 3200, 4000,
        5000, 6400, 8000, 10000, 12800, 16000, 20000, 25600,
    ];
}

#[async_trait]
impl Module for BluetoothCamera {
    fn name(&self) -> String {
        return S("camera");
    }

    async fn trigger(&mut self, action: &str) -> Option<String> {
        match action {
            "iso_up" => iso(self, "up").await,
            "iso_down" => iso(self, "down").await,
            "wb_up" => wb(self, 200).await,
            "wb_down" => wb(self, -200).await,
            _ => None,
        }
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

                        return Some(nv.to_string());
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
