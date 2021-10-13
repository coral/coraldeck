use super::Renderer;
use crate::sman::StreamDeckManager;

mod bigletter;
mod loadseq;

pub struct Boot<'gfx> {
    sman: &'gfx mut StreamDeckManager,
    renderer: Renderer,

    loadcount: u32,
}

impl<'gfx> Boot<'gfx> {
    pub fn new(sman: &'gfx mut StreamDeckManager) -> Boot<'gfx> {
        Boot {
            sman,
            renderer: Renderer::new(72, 72),

            loadcount: 0,
        }
    }

    pub async fn header(&mut self) {
        //Clear
        let _ = self.sman.reset().await;

        for (index, c) in "CDECK".to_string().chars().enumerate() {
            let img = self
                .renderer
                .draw(Box::new(bigletter::BigLetter::new(&c.to_string())))
                .await
                .unwrap();

            let _ = self.sman.set_button_image(index as u8, img).await;
        }
    }

    pub async fn load(&mut self, module: &str) {
        let img = self
            .renderer
            .draw(Box::new(loadseq::LoadSeq::new(module)))
            .await
            .unwrap();

        let _ = self
            .sman
            .set_button_image((self.loadcount + 5) as u8, img)
            .await;

        self.loadcount = self.loadcount + 1;
    }
}
