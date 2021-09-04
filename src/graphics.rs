use font_kit::family_name::FamilyName;
use font_kit::font::Font;
use font_kit::properties::{Properties, Weight};
use font_kit::source::SystemSource;
use image::{DynamicImage, ImageBuffer, Rgb, RgbImage};
use raqote::*;

use lazy_static::lazy_static;

lazy_static! {
    static ref FONT: Font = SystemSource::new()
        .select_by_postscript_name("Helvetica")
        .unwrap()
        .load()
        .unwrap();
}

pub struct Drawer {
    dt: DrawTarget,

    bg: SolidSource,
    text_bg: SolidSource,
    border: SolidSource,
    border_accent: SolidSource,

    text: SolidSource,
    category_text: SolidSource,
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

            border: SolidSource {
                r: 117,
                g: 117,
                b: 117,
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
        }
    }
}

impl Drawer {
    pub fn new() -> Drawer {
        Drawer::default()
    }

    pub fn draw(&mut self, header: &str, action: &str, value: &str) -> DynamicImage {
        self.header(header);
        self.content(action, value);
        self.output()
    }

    pub fn output(&mut self) -> DynamicImage {
        let data = self.dt.get_data();
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

        //Border
        let mut pb = PathBuilder::new();
        pb.rect(0., 16., 72., 2.);
        self.dt.fill(
            &pb.finish(),
            &Source::Solid(self.border),
            &DrawOptions::new(),
        );

        //Border Accent
        let mut pb = PathBuilder::new();
        pb.rect(0., 16., 15., 2.);
        self.dt.fill(
            &pb.finish(),
            &Source::Solid(self.border_accent),
            &DrawOptions::new(),
        );

        //Category text
        self.dt.draw_text(
            &FONT,
            12.,
            header_text,
            Point::new(6., 12.),
            &Source::Solid(self.category_text),
            &DrawOptions::new(),
        );
    }

    fn content(&mut self, action: &str, value: &str) {
        //Action Text
        self.dt.draw_text(
            &FONT,
            20.,
            action,
            Point::new(5., 42.),
            &Source::Solid(self.text),
            &DrawOptions::new(),
        );

        //Value Text
        self.dt.draw_text(
            &FONT,
            20.,
            value,
            Point::new(5., 65.),
            &Source::Solid(self.text),
            &DrawOptions::new(),
        );
    }
}
