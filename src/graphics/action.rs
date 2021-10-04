use crate::graphics;
use image::DynamicImage;
use raqote::*;

use super::{ButtonRenderer, FontLoader};

pub struct Action {
    bg: SolidSource,
    text_bg: SolidSource,
    border_accent: SolidSource,

    text: SolidSource,
    category_text: SolidSource,

    grdbg: SolidSource,

    header: String,
    action: String,
    value: String,
}

impl Default for Action {
    fn default() -> Self {
        Self {
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

            header: format!(""),
            action: format!(""),
            value: format!(""),
        }
    }
}

impl ButtonRenderer for Action {
    fn render(&self, dt: &mut DrawTarget, fonts: &FontLoader) -> DynamicImage {
        self.header(dt, fonts, &self.header);
        self.content(dt, fonts, &self.action, &self.value);

        graphics::output(dt.get_data())
    }
}

impl Action {
    pub fn new(header_color: graphics::Color, header: &str, action: &str, value: &str) -> Action {
        let mut d = Action::default();
        d.border_accent = SolidSource {
            r: header_color.r,
            g: header_color.g,
            b: header_color.b,
            a: 0xff,
        };

        d.header = header.to_string();
        d.action = action.to_string();
        d.value = value.to_string();

        d
    }

    fn header(&self, dt: &mut DrawTarget, font: &FontLoader, header_text: &str) {
        let mut pb = PathBuilder::new();
        //Background
        pb.rect(0., 0., 72., 72.);
        dt.fill(&pb.finish(), &Source::Solid(self.bg), &DrawOptions::new());

        //Text bg
        let mut pb = PathBuilder::new();
        pb.rect(0., 0., 72., 15.);
        dt.fill(
            &pb.finish(),
            &Source::Solid(self.text_bg),
            &DrawOptions::new(),
        );

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
        dt.fill(&pb.finish(), &gradient, &DrawOptions::new());

        //Category text
        dt.draw_text(
            &font.normal,
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
        dt.fill(&pb.finish(), &gradient, &DrawOptions::new());
    }

    fn content(&self, dt: &mut DrawTarget, font: &FontLoader, action: &str, value: &str) {
        //Action Text
        dt.draw_text(
            &font.normal,
            16.,
            action,
            Point::new(5., 39.),
            &Source::Solid(self.text),
            &DrawOptions::new(),
        );

        //Value Text
        dt.draw_text(
            &font.bold,
            16.,
            value,
            Point::new(5., 63.),
            &Source::Solid(self.text),
            &DrawOptions::new(),
        );
    }
}
