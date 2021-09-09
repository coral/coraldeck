use crate::config::{Actions, Config};
use crate::graphics::{Color, Drawer};
use crate::modules::Module;
use crate::StreamDeckManager;
use big_s::S;
use image::{ImageBuffer, Rgb};
use log::trace;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::mpsc::{self, Receiver, Sender};
use tokio::sync::Mutex;

pub struct Controller {
    cfg: Arc<Config>,
    sman: StreamDeckManager,

    index: HashMap<u8, Actions>,
    modules: HashMap<String, Box<dyn Module + Send>>,

    buttons: Arc<Mutex<Vec<Button>>>,
    values: Arc<Mutex<HashMap<String, String>>>,
    rendtrig: Sender<bool>,
}

pub struct Button {
    index: u8,
    module: String,
    color: Color,
    action: String,
    display: Option<String>,
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
        let (rend_tx, rend_rx) = mpsc::channel(32);

        let mut ctrl = Controller {
            cfg,
            sman,
            index: HashMap::new(),
            modules: HashMap::new(),

            buttons: Arc::new(Mutex::new(Vec::new())),
            values: Arc::new(Mutex::new(HashMap::new())),
            rendtrig: rend_tx,
        };

        ctrl.setup(modules).await;

        let render_deck = ctrl.sman.clone();
        let render_list = ctrl.buttons.clone();
        let render_values = ctrl.values.clone();

        tokio::spawn(async move {
            Controller::render(render_deck, render_list, render_values, rend_rx).await;
        });

        ctrl
    }

    async fn setup(&mut self, mut modules: Vec<ModuleConfig>) {
        let _ = self.sman.reset().await;

        //Setup routing
        for action in &self.cfg.actions {
            self.index.insert(action.btn, action.clone());
        }

        let mut sb = self.buttons.lock().await;
        for action in &self.cfg.actions {
            let color = match modules.iter().find(|&x| x.module.name() == action.module) {
                Some(v) => v.color,
                None => Color {
                    r: 100,
                    g: 100,
                    b: 100,
                },
            };

            sb.push(Button {
                index: action.btn,
                module: action.module.to_uppercase(),
                color: color,
                action: action.desc.clone(),
                display: match &action.display {
                    Some(v) => Some(format!("{}_{}", &action.module, &action.value)),
                    None => None,
                },
            });
        }

        //Setup modules
        let mut min: HashMap<String, Box<dyn Module + Send>> = HashMap::new();
        for mc in modules.into_iter() {
            let name = mc.module.name();
            min.insert(name, mc.module);
        }

        self.modules = min;
    }

    async fn render(
        mut sman: StreamDeckManager,
        buttons: Arc<Mutex<Vec<Button>>>,
        values: Arc<Mutex<HashMap<String, String>>>,
        mut trig: Receiver<bool>,
    ) {
        loop {
            {
                let btnstate = buttons.lock().await;
                let values = values.lock().await;

                for button in btnstate.iter() {
                    {
                        let dispval = match &button.display {
                            Some(k) => match values.get(k) {
                                Some(v) => v,
                                None => "",
                            },
                            None => "",
                        };

                        let img = Drawer::newdraw(
                            button.color,
                            &button.module.to_uppercase(),
                            &button.action,
                            dispval,
                        );
                        let _ = sman.set_button_image(button.index, img).await;
                    }
                }
            }
            let _ = trig.recv().await;
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
                    trace!("Trigger {} for {}", &act.action, &act.module);
                    match v.trigger(&act.action).await {
                        Some(newvalue) => {
                            self.values
                                .lock()
                                .await
                                .insert(format!("{}_{}", &act.module, &act.value), newvalue);
                        }
                        None => {}
                    }
                }
                None => println!("Notfound"),
            };

            let _ = self.rendtrig.send(true).await;
        }
    }
}
