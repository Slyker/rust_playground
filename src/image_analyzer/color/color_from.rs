use super::hsv::Hsv;
use super::rgb::Rgb;

use super::conversion::*;
/// From Rgb to Hsv conversion
impl From<Rgb> for Hsv {
    fn from(rgb: Rgb) -> Self {
        hsv_from_rgb(&rgb)
    }
}

/// From Hsv to Rgb conversion
impl From<Hsv> for Rgb {
    fn from(hsv: Hsv) -> Self {
        rgb_from_hsv(&hsv)
    }
}

impl From<&Rgb> for Hsv {
    fn from(rgb: &Rgb) -> Self {
        hsv_from_rgb(rgb)
    }
}

impl From<&Hsv> for Rgb {
    fn from(hsv: &Hsv) -> Self {
        rgb_from_hsv(hsv)
    }
}
