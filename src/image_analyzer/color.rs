use serde::{Deserialize, Serialize};

pub mod color_from;
pub mod conversion;
pub mod hsv;
pub mod rgb;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Color {
    Rgb(rgb::Rgb),
    Hsv(hsv::Hsv),
}

impl Color {
    #[allow(dead_code)]
    pub fn get_rgb(&self) -> rgb::Rgb {
        match self {
            Color::Rgb(rgb) => rgb.to_owned(),
            Color::Hsv(hsv) => rgb::Rgb::from(hsv),
        }
    }
    #[allow(dead_code)]
    pub fn get_hsv(&self) -> hsv::Hsv {
        match self {
            Color::Rgb(rgb) => hsv::Hsv::from(rgb),
            Color::Hsv(hsv) => hsv.to_owned(),
        }
    }
}
