use crate::graphics;
use image::DynamicImage;
use raqote::*;

use crate::graphics::{ButtonRenderer, FontLoader};

pub struct BigLetter {
    bg: SolidSource,

    header_text: SolidSource,

    letter: String,
}

impl Default for BigLetter {
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

            letter: "".to_string(),
        }
    }
}

impl ButtonRenderer for BigLetter {
    fn render(&self, dt: &mut DrawTarget, fonts: &FontLoader) -> DynamicImage {
        let mut pb = PathBuilder::new();
        pb.rect(0., 0., 72., 72.);
        dt.fill(&pb.finish(), &Source::Solid(self.bg), &DrawOptions::new());

        dt.draw_text(
            &fonts.bold,
            24.,
            &self.letter,
            Point::new(30., 40.),
            &Source::Solid(self.header_text),
            &DrawOptions::new(),
        );

        graphics::output(dt.get_data())
    }
}

impl BigLetter {
    pub fn new(letter: &str) -> BigLetter {
        let mut d = BigLetter::default();
        d.letter = letter.to_string();
        d
    }
}
