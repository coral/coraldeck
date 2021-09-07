use crate::config::{Actions, Config};
use crate::graphics::{Color, Drawer};
use crate::modules::Module;
use crate::StreamDeckManager;
use image::{ImageBuffer, Rgb};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct Controller {
    cfg: Arc<Config>,
    sman: StreamDeckManager,

    index: HashMap<u8, Actions>,
    modules: HashMap<String, Box<dyn Module + Send>>,

    buttons: Arc<Mutex<Vec<Button>>>,
}

pub struct Button {
    index: u8,
    module: String,
    color: Color,
    action: String,
    value: String,
}

pub struct ModuleConfig {
    pub module: Box<dyn Module + Send>,
    pub color: Color,
}

impl Controller {
    pub async fn new(
        cfg: Arc<Config>,
        sman: StreamDeckManager,
        modules: Vec<ModuleConfig>,
    ) -> Controller {
        let mut ctrl = Controller {
            cfg,
            sman,
            index: HashMap::new(),
            modules: HashMap::new(),

            buttons: Arc::new(Mutex::new(Vec::new())),
        };

        ctrl.setup(modules).await;

        let render_deck = ctrl.sman.clone();
        let render_list = ctrl.buttons.clone();

        tokio::spawn(async move {
            Controller::render(render_deck, render_list).await;
        });

        ctrl
    }

    async fn setup(&mut self, mut modules: Vec<ModuleConfig>) {
        self.sman.reset().await;

        //Setup routing
        for action in &self.cfg.actions {
            self.index.insert(action.btn, action.clone());
        }

        let mut sb = self.buttons.lock().await;
        for action in &self.cfg.actions {
            sb.push(Button {
                index: action.btn,
                module: action.module.to_uppercase(),
                color: modules
                    .iter()
                    .find(|&x| x.module.name() == action.module)
                    .unwrap()
                    .color
                    .clone(),
                action: action.desc.clone(),
                value: "".to_string(),
            });
        }

        //Setup modules
        let mut min: HashMap<String, Box<dyn Module + Send>> = HashMap::new();
        for mc in modules.into_iter() {
            let name = mc.module.name();
            min.insert(name, mc.module);
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

    async fn render(mut sman: StreamDeckManager, buttons: Arc<Mutex<Vec<Button>>>) {
        loop {
            let btnstate = buttons.lock().await;
            for button in btnstate.iter() {
                let mut drw = Drawer::new();
                sman.set_button_image(
                    button.index,
                    drw.draw(&button.module.to_uppercase(), &button.action, &button.value),
                )
                .await;
            }
        }
    }

    pub async fn spin(&mut self) {
        let mut events = self.sman.subscribe().await;

        loop {
            let event = events.recv().await.unwrap();

            let act = match self.index.get(&event.num) {
                Some(v) => v,
                None => continue,
            };

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
