use crate::config::Config;
use image::{ImageBuffer, Rgb};
use std::sync::Arc;

struct Buffer {
    normal: ImageBuffer<Rgb<u8>, Vec<u8>>,
    pressed: ImageBuffer<Rgb<u8>, Vec<u8>>,
}
pub struct Controller {
    cfg: Arc<Config>,
}

impl Controller {
    pub async fn new(cfg: Arc<Config>) -> Controller {
        let mut ctrl = Controller { cfg };

        ctrl.setup().await;

        ctrl
    }

    async fn setup(&mut self) {
        for action in &self.cfg.actions {}
    }

    pub async fn spin(&mut self) {}
}
