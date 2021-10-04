use font_kit::font::Font;

pub struct FontLoader {
    pub normal: Font,
    pub bold: Font,
}

impl FontLoader {
    pub fn new() -> FontLoader {
        FontLoader {
            normal: font_kit::loader::Loader::from_path("fonts/OpenSans-Medium.ttf", 0).unwrap(),
            bold: font_kit::loader::Loader::from_path("fonts/OpenSans-Bold.ttf", 0).unwrap(),
        }
    }
}
