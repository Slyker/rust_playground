use serde::{Deserialize, Serialize};

use crate::data::point::Point;

use super::color::Color;
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Pixel {
    pub color: Color,
    pub points: Vec<Point>,
}

impl Pixel {
    pub fn extend(&mut self, other: &Pixel) {
        self.points.extend(other.points.iter().cloned());
    }
}

pub type PixelVec = Vec<Pixel>;
pub struct _PixelVec {
    pub pixels: Vec<Pixel>,
    pub points_count: usize,
}

impl _PixelVec {
    pub fn new() -> Self {
        Self { pixels: Vec::new(), points_count: 0 }
    }

    pub fn push(&mut self, (color,point): (Color, Point)) {
        self.points_count += 1;
        if let Some(pixel) = self.pixels.iter_mut().find(|pixel| pixel.color == color) {
            pixel.points.push(point);
            return;
        }
        self.pixels.push(Pixel {
            color,
            points: vec![point],
        });
    }
}