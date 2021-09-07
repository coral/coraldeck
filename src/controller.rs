use crate::config::{Actions, Config};
use crate::graphics::Drawer;
use crate::modules::Module;
use crate::StreamDeckManager;
use image::{ImageBuffer, Rgb};
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::Arc;

struct Buffer {
    normal: ImageBuffer<Rgb<u8>, Vec<u8>>,
    pressed: ImageBuffer<Rgb<u8>, Vec<u8>>,
}
pub struct Controller {
    cfg: Arc<Config>,
    sman: StreamDeckManager,

    index: HashMap<u8, Actions>,
    modules: HashMap<String, Box<dyn Module + Send>>,
}

impl Controller {
    pub async fn new(
        cfg: Arc<Config>,
        sman: StreamDeckManager,
        modules: Vec<Box<dyn Module + Send>>,
    ) -> Controller {
        let mut ctrl = Controller {
            cfg,
            sman,
            index: HashMap::new(),
            modules: HashMap::new(),
        };

        //let mut call: HashMap<String, Box<dyn Module>> = HashMap::new();

        ctrl.setup(modules).await;

        ctrl
    }

    async fn setup(&mut self, mut modules: Vec<Box<dyn Module + Send>>) {
        self.sman.reset().await;

        //Setup routing
        for action in &self.cfg.actions {
            self.index.insert(action.btn, action.clone());
        }

        //Setup modules
        let mut min: HashMap<String, Box<dyn Module + Send>> = HashMap::new();
        for module in modules.into_iter() {
            let name = module.name();
            min.insert(name, module);
        }

        self.modules = min;

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

    pub async fn spin(&mut self) {
        let mut events = self.sman.subscribe().await;

        loop {
            let event = events.recv().await.unwrap();

            let act = self.index.get(&event.num).unwrap();

            let m = self.modules.get_mut(&act.module);
            match m {
                Some(v) => {
                    v.trigger(&act.action).await;
                }
                None => println!("Notfound"),
            };
        }
    }
}
