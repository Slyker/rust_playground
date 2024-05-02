use serde::{Serialize, Deserialize};

use super::hsv::Hsv;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy, Hash)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Rgb {
    pub fn diff(&self, other: &Rgb) -> Rgb {
        Rgb {
            r: (self.r as i16 - other.r as i16).abs() as u8,
            g: (self.g as i16 - other.g as i16).abs() as u8,
            b: (self.b as i16 - other.b as i16).abs() as u8,
            a: (self.a as i16 - other.a as i16).abs() as u8,
        }
    }

    pub fn compare(&self, other: &Rgb, tolerance: Rgb) -> bool {
        let diff = self.diff(other);
        diff.r <= tolerance.r && diff.g <= tolerance.g && diff.b <= tolerance.b && diff.a <= tolerance.a
    }

    pub fn compare_from_hsv(&self, other: &Hsv, tolerance: Rgb) -> bool {
        let diff = self.diff(&Rgb::from(other));
        diff.r <= tolerance.r && diff.g <= tolerance.g && diff.b <= tolerance.b && diff.a <= tolerance.a
    }

    pub fn compare_to_hsv(&self, other: &Hsv, tolerance: Hsv) -> bool {
        let hsv = Hsv::from(self);
        hsv.compare(other, tolerance)
    }
}

impl From<[u8; 3]> for Rgb {
    fn from(rgb: [u8; 3]) -> Self {
        let rgb = Rgb {
            r: rgb[0],
            g: rgb[1],
            b: rgb[2],
            a: 255, // Opaque
        };
        rgb
    }
}

impl From<[u8; 4]> for Rgb {
    fn from(rgba: [u8; 4]) -> Self {
        let rgb = Rgb {
            r: rgba[0],
            g: rgba[1],
            b: rgba[2],
            a: rgba[3],
        };
        rgb
    }
}

