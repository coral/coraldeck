use crate::graphics;
use font_kit::family_name::FamilyName;
use font_kit::font::Font;
use font_kit::properties::{Properties, Weight};
use font_kit::source::SystemSource;
use image::DynamicImage;
use raqote::*;
use std::sync::{Arc, Mutex};

use lazy_static::lazy_static;

lazy_static! {
    static ref FONT: Arc<Mutex<Font>> = Arc::new(Mutex::new(
        SystemSource::new()
            .select_best_match(
                &[FamilyName::Title("Helvetica".into())],
                &Properties::new().weight(Weight::MEDIUM),
            )
            .unwrap()
            .load()
            .unwrap()
    ));
    static ref BOLDFONT: Arc<Mutex<Font>> = Arc::new(Mutex::new(
        SystemSource::new()
            .select_best_match(
                &[FamilyName::Title("Helvetica".into())],
                &Properties::new().weight(Weight::BOLD),
            )
            .unwrap()
            .load()
            .unwrap()
    ));
}

pub struct Drawer {
    dt: DrawTarget,

    bg: SolidSource,
    text_bg: SolidSource,
    border_accent: SolidSource,

    text: SolidSource,
    category_text: SolidSource,

    grdbg: SolidSource,
}

impl Default for Drawer {
    fn default() -> Self {
        Self {
            dt: DrawTarget::new(72, 72),

            bg: SolidSource {
                r: 40,
                g: 40,
                b: 40,
                a: 0xff,
            },

            text_bg: SolidSource {
                r: 25,
                g: 25,
                b: 25,
                a: 0xff,
            },

            border_accent: SolidSource {
                r: 255,
                g: 100,
                b: 0,
                a: 0xff,
            },

            text: SolidSource {
                r: 160,
                g: 160,
                b: 160,
                a: 0xff,
            },

            category_text: SolidSource {
                r: 160,
                g: 160,
                b: 160,
                a: 0xff,
            },

            grdbg: SolidSource {
                r: 10,
                g: 10,
                b: 10,
                a: 0xff,
            },
        }
    }
}

impl Drawer {
    pub fn draw(&mut self, header: &str, action: &str, value: &str) -> DynamicImage {
        self.header(header);
        self.content(action, value);
        graphics::output(self.dt.get_data())
    }

    pub fn newdraw(
        header_color: graphics::Color,
        header: &str,
        action: &str,
        value: &str,
    ) -> DynamicImage {
        let mut d = Drawer::default();
        d.border_accent = SolidSource {
            r: header_color.r,
            g: header_color.g,
            b: header_color.b,
            a: 0xff,
        };
        d.draw(header, action, value);
        graphics::output(d.dt.get_data())
    }

    fn header(&mut self, header_text: &str) {
        let mut pb = PathBuilder::new();
        //Background
        pb.rect(0., 0., 72., 72.);
        self.dt
            .fill(&pb.finish(), &Source::Solid(self.bg), &DrawOptions::new());

        //Text bg
        let mut pb = PathBuilder::new();
        pb.rect(0., 0., 72., 15.);
        self.dt.fill(
            &pb.finish(),
            &Source::Solid(self.text_bg),
            &DrawOptions::new(),
        );

        // let c = match COLORS.get(header_text) {
        //     Some(v) => *v,
        //     None => Color::new(255, 255, 100, 0),
        // };

        let c = Color::new(
            self.border_accent.a,
            self.border_accent.r,
            self.border_accent.g,
            self.border_accent.b,
        );
        //Border Accent
        let gradient = Source::new_linear_gradient(
            Gradient {
                stops: vec![
                    GradientStop {
                        position: 0.0,
                        color: c,
                    },
                    GradientStop {
                        position: 1.0,
                        color: Color::new(255, 70, 70, 70),
                    },
                ],
            },
            Point::new(0., 0.),
            Point::new(72., 0.),
            Spread::Pad,
        );
        let mut pb = PathBuilder::new();
        pb.rect(0., 17., 72., 4.);
        self.dt.fill(&pb.finish(), &gradient, &DrawOptions::new());

        //Category text
        self.dt.draw_text(
            &FONT.clone().lock().unwrap(),
            12.,
            header_text,
            Point::new(6., 12.),
            &Source::Solid(self.category_text),
            &DrawOptions::new(),
        );

        //Border Accent
        let gradient = Source::new_linear_gradient(
            Gradient {
                stops: vec![
                    GradientStop {
                        position: 0.0,
                        color: Color::new(self.bg.a, self.bg.r, self.bg.g, self.bg.b),
                    },
                    GradientStop {
                        position: 1.0,
                        color: Color::new(self.grdbg.a, self.grdbg.r, self.grdbg.g, self.grdbg.b),
                    },
                ],
            },
            Point::new(0., 0.),
            Point::new(0., 52.),
            Spread::Pad,
        );
        let mut pb = PathBuilder::new();
        pb.rect(0., 20., 72., 52.);
        self.dt.fill(&pb.finish(), &gradient, &DrawOptions::new());
    }

    fn content(&mut self, action: &str, value: &str) {
        //Action Text
        self.dt.draw_text(
            &FONT.clone().lock().unwrap(),
            16.,
            action,
            Point::new(5., 39.),
            &Source::Solid(self.text),
            &DrawOptions::new(),
        );

        //Value Text
        self.dt.draw_text(
            &BOLDFONT.clone().lock().unwrap(),
            16.,
            value,
            Point::new(5., 63.),
            &Source::Solid(self.text),
            &DrawOptions::new(),
        );
    }
}
