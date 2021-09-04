use font_kit::family_name::FamilyName;
use font_kit::properties::{Properties, Weight};
use font_kit::source::SystemSource;
use image::{ImageBuffer, Pixel, Rgb, RgbImage, Rgba, RgbaImage};
use raqote::*;

pub fn render() -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let mut dt = DrawTarget::new(72, 72);

    let font = SystemSource::new()
        .select_by_postscript_name("Helvetica")
        .unwrap()
        .load()
        .unwrap();

    println!("{:?}", font);

    let mut pb = PathBuilder::new();
    //Background
    pb.rect(0., 0., 72., 72.);
    dt.fill(
        &pb.finish(),
        &Source::Solid(SolidSource {
            r: 40,
            g: 40,
            b: 40,
            a: 0xff,
        }),
        &DrawOptions::new(),
    );

    //Text bg
    let mut pb = PathBuilder::new();
    pb.rect(0., 0., 72., 15.);
    dt.fill(
        &pb.finish(),
        &Source::Solid(SolidSource {
            r: 25,
            g: 25,
            b: 25,
            a: 0xff,
        }),
        &DrawOptions::new(),
    );

    //Border
    let mut pb = PathBuilder::new();
    pb.rect(0., 16., 72., 2.);
    dt.fill(
        &pb.finish(),
        &Source::Solid(SolidSource {
            r: 117,
            g: 117,
            b: 117,
            a: 0xff,
        }),
        &DrawOptions::new(),
    );

    //Border Accent
    let mut pb = PathBuilder::new();
    pb.rect(0., 16., 15., 2.);
    dt.fill(
        &pb.finish(),
        &Source::Solid(SolidSource {
            r: 255,
            g: 100,
            b: 0,
            a: 0xff,
        }),
        &DrawOptions::new(),
    );

    //Category text
    dt.draw_text(
        &font,
        12.,
        "CAMERA",
        Point::new(6., 12.),
        &Source::Solid(SolidSource {
            r: 160,
            g: 160,
            b: 160,
            a: 0xff,
        }),
        &DrawOptions::new(),
    );

    //Action Text
    dt.draw_text(
        &font,
        20.,
        "ISO +",
        Point::new(5., 42.),
        &Source::Solid(SolidSource {
            r: 160,
            g: 160,
            b: 160,
            a: 0xff,
        }),
        &DrawOptions::new(),
    );

    //Value Text
    dt.draw_text(
        &font,
        20.,
        "1600",
        Point::new(5., 65.),
        &Source::Solid(SolidSource {
            r: 160,
            g: 160,
            b: 160,
            a: 0xff,
        }),
        &DrawOptions::new(),
    );

    convert(dt.get_data())
}

fn convert(data: &[u32]) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    RgbImage::from_fn(72, 72, |x, y| {
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
    })
}
