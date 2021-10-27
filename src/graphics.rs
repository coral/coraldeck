mod action;
mod boot;
mod fontloader;

use crate::error::Error;
pub use action::Action;
pub use boot::Boot;
pub use fontloader::FontLoader;
use raqote::DrawTarget;
use serde::{Deserialize, Serialize};
use tokio::runtime::Builder;
use tokio::sync::{mpsc, mpsc::UnboundedReceiver, mpsc::UnboundedSender, oneshot};
use tokio::task::LocalSet;

use image::{DynamicImage, Rgb, RgbImage};

pub fn output(data: &[u32]) -> DynamicImage {
    let ni = RgbImage::from_fn(72, 72, |x, y| {
        let pixel = data[((72 * y) + x) as usize];
        let a = (pixel >> 24) & 0xffu32;
        let mut r = (pixel >> 16) & 0xffu32;
        let mut g = (pixel >> 8) & 0xffu32;
        let mut b = (pixel >> 0) & 0xffu32;

        if a > 0u32 {
            r = r * 255u32 / a;
            g = g * 255u32 / a;
            b = b * 255u32 / a;
        }

        Rgb {
            0: [r as u8, g as u8, b as u8],
        }
    });

    DynamicImage::ImageRgb8(ni)
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub struct DrawJob {
    job: Box<dyn ButtonRenderer + Send>,
    completion: oneshot::Sender<Result<DynamicImage, Error>>,
}

#[derive(Clone)]
pub struct Renderer {
    task_queue: UnboundedSender<DrawJob>,
}

impl Renderer {
    pub fn new(width: i32, height: i32) -> Renderer {
        let rt = Builder::new_current_thread().enable_all().build().unwrap();

        let (task_send, mut task_recv): (UnboundedSender<DrawJob>, UnboundedReceiver<DrawJob>) =
            mpsc::unbounded_channel();

        std::thread::spawn(move || {
            let local = LocalSet::new();

            local.spawn_local(async move {
                let fonts = FontLoader::new();

                loop {
                    match task_recv.recv().await {
                        Some(new_task) => {
                            let mut dt = DrawTarget::new(width, height);
                            let img = new_task.job.as_ref().render(&mut dt, &fonts);
                            let _ = new_task.completion.send(Ok(img));
                        }
                        None => return,
                    }
                }
            });
            rt.block_on(local);
        });

        Renderer {
            task_queue: task_send,
        }
    }

    pub async fn draw(&self, job: Box<dyn ButtonRenderer + Send>) -> Result<DynamicImage, Error> {
        let (compl_tx, compl_rx) = oneshot::channel();

        match self.task_queue.send(DrawJob {
            job,
            completion: compl_tx,
        }) {
            Ok(_) => {}
            Err(_) => return Err(Error::RenderCrash),
        }

        let img = compl_rx.await;
        img?
    }
}

pub trait ButtonRenderer {
    fn render(&self, dt: &mut DrawTarget, fonts: &FontLoader) -> DynamicImage;
}
