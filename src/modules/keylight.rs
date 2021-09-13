use crate::modules::Module;
use async_trait::async_trait;
use big_s::S;
pub use elgato_keylight::KeyLight;
use tokio::sync::mpsc::Sender;

pub struct KeyLights {
    lights: Vec<KeyLight>,
    rendtrig: Option<Sender<(String, String)>>,
}

impl KeyLights {
    pub async fn new(lights: Vec<KeyLight>) -> KeyLights {
        KeyLights {
            lights,
            rendtrig: None,
        }
    }

    async fn toggle(&mut self) -> String {
        let ns = self.lights[0].get().await.unwrap().lights[0].on == 0;
        if ns {
            S("power_on")
        } else {
            S("power_off")
        }
    }
}

#[async_trait]
impl Module for KeyLights {
    fn name(&self) -> String {
        return S("keylight");
    }

    async fn trigger(&mut self, action: &str) -> Option<String> {
        match action {
            "left_light_up" => self.lights[0].trigger("light_up").await,
            "left_light_down" => self.lights[0].trigger("light_down").await,
            "right_light_up" => self.lights[1].trigger("light_up").await,
            "right_light_down" => self.lights[1].trigger("light_down").await,
            "toggle_power" => {
                let p = self.toggle().await;
                let mut res: Option<String> = None;
                for l in self.lights.iter_mut() {
                    res = l.trigger(&p).await
                }

                res
            }
            _ => None,
        }
    }

    async fn subscribe(&mut self) -> tokio::sync::mpsc::Receiver<(String, String)> {
        let (tx, rx) = tokio::sync::mpsc::channel(16);

        self.rendtrig = Some(tx.clone());

        let _ = tx
            .send((
                S("keylight_left_intensity"),
                format!(
                    "{}%",
                    self.lights[0].get().await.unwrap().lights[0].brightness
                )
                .to_string(),
            ))
            .await;

        let _ = tx
            .send((
                S("keylight_right_intensity"),
                format!(
                    "{}%",
                    self.lights[1].get().await.unwrap().lights[0].brightness
                )
                .to_string(),
            ))
            .await;

        rx
    }
}

#[async_trait]
impl Module for KeyLight {
    fn name(&self) -> String {
        return S("keylight");
    }

    async fn trigger(&mut self, action: &str) -> Option<String> {
        match action {
            "light_up" => Some(format!(
                "{}%",
                self.set_relative_brightness(0.02)
                    .await
                    .unwrap()
                    .to_string()
            )),
            "light_down" => Some(format!(
                "{}%",
                self.set_relative_brightness(-0.02)
                    .await
                    .unwrap()
                    .to_string()
            )),
            "power_on" => match self.set_power(true).await {
                Ok(_) => Some(S("ON")),
                Err(_) => None,
            },
            "power_off" => match self.set_power(false).await {
                Ok(_) => Some(S("OFF")),
                Err(_) => None,
            },

            _ => None,
        }
    }

    async fn subscribe(&mut self) -> tokio::sync::mpsc::Receiver<(String, String)> {
        let (tx, rx) = tokio::sync::mpsc::channel(16);

        rx
    }
}
