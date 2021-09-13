mod drawer;
mod startup;

pub use drawer::Drawer;
use serde::{Deserialize, Serialize};
pub use startup::Startup;

use image::{DynamicImage, Rgb, RgbImage};

pub fn output(data: &[u32]) -> DynamicImage {
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

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}
