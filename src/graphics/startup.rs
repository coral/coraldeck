use crate::graphics;
use font_kit::source::SystemSource;
use raqote::*;

use crate::sman::StreamDeckManager;

pub struct Startup {
    bg: SolidSource,

    header_text: SolidSource,
    loaded_text: SolidSource,
    text: SolidSource,

    loadcount: u32,
}

impl Default for Startup {
    fn default() -> Self {
        Self {
            bg: SolidSource {
                r: 40,
                g: 40,
                b: 40,
                a: 0xff,
            },

            header_text: SolidSource {
                r: 160,
                g: 160,
                b: 160,
                a: 0xff,
            },

            loaded_text: SolidSource {
                r: 120,
                g: 120,
                b: 120,
                a: 0xff,
            },

            text: SolidSource {
                r: 180,
                g: 180,
                b: 180,
                a: 0xff,
            },

            loadcount: 0,
        }
    }
}

impl Startup {
    pub async fn new(sman: &mut StreamDeckManager) -> Startup {
        let mut d = Startup::default();
        let _ = sman.reset().await;
        d.draw_header(sman).await;

        d
    }

    async fn draw_header(&mut self, sman: &mut StreamDeckManager) {
        let font = SystemSource::new()
            .select_by_postscript_name("Helvetica")
            .unwrap()
            .load()
            .unwrap();

        for (index, c) in "CDECK".to_string().chars().enumerate() {
            let mut dt = DrawTarget::new(72, 72);
            let mut pb = PathBuilder::new();
            //Background
            pb.rect(0., 0., 72., 72.);
            dt.fill(&pb.finish(), &Source::Solid(self.bg), &DrawOptions::new());

            dt.draw_text(
                &font,
                24.,
                &c.to_string(),
                Point::new(30., 40.),
                &Source::Solid(self.header_text),
                &DrawOptions::new(),
            );

            let _ = sman
                .set_button_image(index as u8, graphics::output(dt.get_data()))
                .await;
        }
    }

    pub async fn load(&mut self, sman: &mut StreamDeckManager, module: &str) {
        let font = SystemSource::new()
            .select_by_postscript_name("Helvetica")
            .unwrap()
            .load()
            .unwrap();

        let mut dt = DrawTarget::new(72, 72);
        let mut pb = PathBuilder::new();
        //Background
        pb.rect(0., 0., 72., 72.);
        dt.fill(&pb.finish(), &Source::Solid(self.bg), &DrawOptions::new());

        //Category text
        dt.draw_text(
            &font,
            16.,
            "LOADED:",
            Point::new(4., 22.),
            &Source::Solid(self.loaded_text),
            &DrawOptions::new(),
        );

        dt.draw_text(
            &font,
            12.,
            module,
            Point::new(4., 42.),
            &Source::Solid(self.text),
            &DrawOptions::new(),
        );

        let _ = sman
            .set_button_image((self.loadcount + 5) as u8, graphics::output(dt.get_data()))
            .await;

        self.loadcount = self.loadcount + 1;
    }
}
