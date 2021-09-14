use font_kit::family_name::FamilyName;
use font_kit::font::Font;
use font_kit::properties::{Properties, Weight};
use font_kit::source::SystemSource;

pub struct FontLoader {
    pub normal: Font,
    pub bold: Font,
}

impl FontLoader {
    pub fn new() -> FontLoader {
        FontLoader {
            normal: SystemSource::new()
                .select_best_match(
                    &[FamilyName::Title("Helvetica".into())],
                    &Properties::new().weight(Weight::MEDIUM),
                )
                .unwrap()
                .load()
                .unwrap(),
            bold: SystemSource::new()
                .select_best_match(
                    &[FamilyName::Title("Helvetica".into())],
                    &Properties::new().weight(Weight::BOLD),
                )
                .unwrap()
                .load()
                .unwrap(),
        }
    }
}
