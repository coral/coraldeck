use coraldeck::graphics;
use coraldeck::sman::StreamDeckManager;
use image::DynamicImage;

use font_kit::family_name::FamilyName;
use font_kit::font::Font;
use font_kit::properties::{Properties, Weight};
use font_kit::source::SystemSource;
use raqote::*;

#[tokio::main]
async fn main() {
    let bg = SolidSource {
        r: 40,
        g: 40,
        b: 40,
        a: 0xff,
    };
    let mut m = StreamDeckManager::new().await.unwrap();

    let mut dt = DrawTarget::new(72, 72);
    let mut pb = PathBuilder::new();
    //Background
    pb.rect(0., 0., 72., 72.);
    dt.fill(&pb.finish(), &Source::Solid(bg), &DrawOptions::new());

    let gradient = Source::new_linear_gradient(
        Gradient {
            stops: vec![
                GradientStop {
                    position: 0.0,
                    color: Color::new(255, 255, 100, 0),
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
    pb.rect(0., 10., 72., 5.);
    dt.fill(&pb.finish(), &gradient, &DrawOptions::new());

    m.set_button_image(0, graphics::output(dt.get_data())).await;
}
