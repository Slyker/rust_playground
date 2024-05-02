use serde::{Deserialize, Serialize};

use crate::data::point::Point;

use super::color::Color;
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Pixel {
    pub color: Color,
    pub points: Vec<Point>,
}
pub struct PixelVec {
    pub pixels: Vec<Pixel>,
    pub points_count: usize,
}

impl PixelVec {
    pub fn new() -> Self {
        Self {
            pixels: Vec::new(),
            points_count: 0,
        }
    }

    pub fn push(&mut self, (color, point): (Color, Point)) {
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
    #[allow(dead_code)]
    pub fn get_by_color(&self, color: &Color) -> Option<&Pixel> {
        self.pixels.iter().find(|pixel| &pixel.color == color)
    }
}
