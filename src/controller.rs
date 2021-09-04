use crate::config::Config;
use crate::graphics::Drawer;
use crate::StreamDeckManager;
use image::{ImageBuffer, Rgb};
use std::sync::Arc;

struct Buffer {
    normal: ImageBuffer<Rgb<u8>, Vec<u8>>,
    pressed: ImageBuffer<Rgb<u8>, Vec<u8>>,
}
pub struct Controller {
    cfg: Arc<Config>,
    sman: StreamDeckManager,
}

impl Controller {
    pub async fn new(cfg: Arc<Config>, sman: StreamDeckManager) -> Controller {
        let mut ctrl = Controller { cfg, sman };

        ctrl.setup().await;

        ctrl
    }

    async fn setup(&mut self) {
        for action in &self.cfg.actions {
            let mut drw = Drawer::new();
            self.sman
                .set_button_image(
                    action.btn,
                    drw.draw(&action.module.to_uppercase(), &action.desc, ""),
                )
                .await;
        }
    }

    pub async fn spin(&mut self) {}
}
