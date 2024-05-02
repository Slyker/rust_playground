use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

impl Point {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }

    pub fn distance(&self, other: &Point) -> f64 {
        let x = (self.x as f64 - other.x as f64).powi(2);
        let y = (self.y as f64 - other.y as f64).powi(2);
        (x + y).sqrt()
    }

    pub fn is_after(&self, point: &Point) -> bool {
        self.x >= point.x || self.y >= point.y // max_x or max_y
    }

    pub fn is_before(&self, point: &Point) -> bool {
        self.x <= point.x || self.y <= point.y // min_x or min_y
    }

    
    pub fn is_under(&self, point: &Point) -> bool {
        self.y >= point.y // max_y
    }
    
    pub fn is_above(&self, point: &Point) -> bool {
        self.y <= point.y // min_y
    }

    pub fn is_inside(&self, point_min: &Point, point_max: &Point) -> bool {
       !self.is_after(point_max) && !self.is_before(point_min)
    }

}