use image::DynamicImage;
use log::error;
use std::sync::Arc;
use std::time::Instant;
use streamdeck::StreamDeck;
use tokio::sync::{broadcast, broadcast::Receiver, broadcast::Sender, Mutex};
use tokio::time::{self, Duration};

#[derive(Clone)]
pub struct StreamDeckManager {
    device: Arc<Mutex<StreamDeck>>,
    channel: Arc<Mutex<Sender<ButtonPress>>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ButtonPress {
    pub num: u8,
    pub timestamp: Instant,
}

#[derive(Debug, Clone)]
struct ButtonState {
    changed: Instant,
}

impl ButtonState {
    fn debounce(&mut self, new_state: bool) -> bool {
        //Debounce
        if self.changed.elapsed() > Duration::from_millis(1) {
            //Check rising
            if new_state {
                self.changed = Instant::now();
                return true;
            }
        }
        false
    }
}

impl Default for ButtonState {
    fn default() -> Self {
        Self {
            changed: Instant::now(),
        }
    }
}

impl StreamDeckManager {
    pub async fn new() -> Result<StreamDeckManager, streamdeck::Error> {
        let mut m = StreamDeck::connect(0x0fd9, 0x0060, None)?;
        m.set_brightness(30)?;
        m.set_blocking(false)?;
        let m = Arc::new(Mutex::new(m));

        let poll_device = m.clone();

        let (tx, _) = broadcast::channel(16);
        let tx = Arc::new(Mutex::new(tx));

        tokio::spawn(StreamDeckManager::poll(poll_device, tx.clone()));

        Ok(StreamDeckManager {
            device: m,
            channel: tx,
        })
    }

    pub async fn subscribe(&mut self) -> Receiver<ButtonPress> {
        self.channel.lock().await.subscribe()
    }

    pub async fn set_button_image(
        &mut self,
        button: u8,
        image: DynamicImage,
    ) -> Result<(), streamdeck::Error> {
        self.device.lock().await.set_button_image(button, image)
    }

    pub async fn reset(&mut self) -> Result<(), streamdeck::Error> {
        self.device.lock().await.reset()
    }

    async fn poll(device: Arc<Mutex<StreamDeck>>, chan: Arc<Mutex<Sender<ButtonPress>>>) {
        let mut interval = time::interval(Duration::from_millis(1));

        let mut state: Vec<ButtonState> = vec![ButtonState::default(); 5 * 3];

        loop {
            interval.tick().await;
            match device.lock().await.read_buttons(None) {
                Ok(data) => {
                    for it in data.iter().enumerate().zip(state.iter_mut()) {
                        let ((num, new_state), stored_state) = it;
                        if stored_state.debounce(*new_state != 0) {
                            match chan.lock().await.send(ButtonPress {
                                num: num as u8,
                                timestamp: Instant::now(),
                            }) {
                                Ok(_) => {}
                                Err(_) => {
                                    error!("Streamdeck event loop could not feed channel");
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }
}
