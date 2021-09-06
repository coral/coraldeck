use crate::modules::Module;
use async_trait::async_trait;
use big_s::S;
pub use elgato_keylight::KeyLight;

pub struct KeyLights {
    lights: Vec<KeyLight>,
}

impl KeyLights {
    pub async fn new(lights: Vec<KeyLight>) -> KeyLights {
        KeyLights { lights }
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
            _ => None,
        }
    }
}

#[async_trait]
impl Module for KeyLight {
    fn name(&self) -> String {
        return S("keylight");
    }

    async fn trigger(&mut self, action: &str) -> Option<String> {
        match action {
            "light_up" => Some(
                self.set_relative_brightness(0.05)
                    .await
                    .unwrap()
                    .to_string(),
            ),
            "light_down" => Some(
                self.set_relative_brightness(-0.05)
                    .await
                    .unwrap()
                    .to_string(),
            ),
            _ => None,
        }
    }
}
