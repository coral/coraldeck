use crate::graphics;
use image::DynamicImage;
use raqote::*;

use crate::graphics::{ButtonRenderer, FontLoader};

pub struct LoadSeq {
    bg: SolidSource,

    loaded_text: SolidSource,
    text: SolidSource,

    module: String,
}

impl Default for LoadSeq {
    fn default() -> Self {
        Self {
            bg: SolidSource {
                r: 40,
                g: 40,
                b: 40,
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

            module: "".to_string(),
        }
    }
}

impl ButtonRenderer for LoadSeq {
    fn render(&self, dt: &mut DrawTarget, fonts: &FontLoader) -> DynamicImage {
        let mut pb = PathBuilder::new();
        //Background
        pb.rect(0., 0., 72., 72.);
        dt.fill(&pb.finish(), &Source::Solid(self.bg), &DrawOptions::new());

        //Category text
        dt.draw_text(
            &fonts.bold,
            16.,
            "LOADED:",
            Point::new(4., 22.),
            &Source::Solid(self.loaded_text),
            &DrawOptions::new(),
        );

        dt.draw_text(
            &fonts.normal,
            12.,
            &self.module,
            Point::new(4., 42.),
            &Source::Solid(self.text),
            &DrawOptions::new(),
        );

        graphics::output(dt.get_data())
    }
}

impl LoadSeq {
    pub fn new(module: &str) -> LoadSeq {
        let mut d = LoadSeq::default();
        d.module = module.to_string();
        d
    }
}
