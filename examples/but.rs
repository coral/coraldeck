use font_kit::family_name::FamilyName;
use font_kit::properties::{Properties, Weight};
use font_kit::source::SystemSource;
use raqote::*;

fn main() {
    let mut dt = DrawTarget::new(72, 72);

    let font = SystemSource::new()
        .select_best_match(&[FamilyName::SansSerif], &Properties::new())
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

    dt.draw_text(
        &font,
        11.,
        "CAMERA",
        Point::new(4., 12.),
        &Source::Solid(SolidSource {
            r: 160,
            g: 160,
            b: 160,
            a: 0xff,
        }),
        &DrawOptions::new(),
    );

    dt.write_png("out.png").unwrap();
}
