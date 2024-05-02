use image::ImageBuffer;

use crate::data::point::Point;

use self::{
    color::{rgb::Rgb, Color},
    pixel::PixelVec,
};

pub mod color;
pub mod pixel;
#[allow(dead_code)]
pub enum LoopResult {
    Continue(Axis),
    Break(Axis),
}
#[allow(dead_code)]
pub enum Axis {
    X,
    Y,
}
#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
pub enum ImageZone {
    Full,
    Partial(Point, Point),
}
pub struct ImageAnalyzer {
    pub image: ImageBuffer<image::Rgba<u8>, Vec<u8>>,
    pub pixels: PixelVec,
}

impl ImageAnalyzer {
    pub fn new(image: ImageBuffer<image::Rgba<u8>, Vec<u8>>) -> Self {
        let pixels = PixelVec::new();

        Self { image, pixels }
    }
    #[allow(dead_code)]
    pub fn set_image(&mut self, image: ImageBuffer<image::Rgba<u8>, Vec<u8>>) {
        self.image = image;
        self.reset_pixels();
    }
    #[allow(dead_code)]
    pub fn reset_pixels(&mut self) {
        self.pixels = PixelVec::new();
    }

    pub fn process_image<F>(&mut self, zone: ImageZone, mut callback: F)
    where
        F: FnMut(Color, Point) -> Option<LoopResult>,
    {
        let width = {
            match zone {
                ImageZone::Full => self.image.width(),
                ImageZone::Partial(ref start, ref end) => {
                    if start.x > end.x
                        || start.y > end.y
                        || end.x > self.image.width()
                        || end.y > self.image.height()
                    {
                        panic!("Invalid zone");
                    }
                    end.x - start.x
                }
            }
        };
        let height = {
            match zone {
                ImageZone::Full => self.image.height(),
                ImageZone::Partial(ref start, ref end) => {
                    if start.x > end.x
                        || start.y > end.y
                        || end.x > self.image.width()
                        || end.y > self.image.height()
                    {
                        panic!("Invalid zone");
                    }
                    end.y - start.y
                }
            }
        };
        let start_y = {
            match zone {
                ImageZone::Full => 0,
                ImageZone::Partial(ref start, _) => start.y,
            }
        };
        let start_x = {
            match zone {
                ImageZone::Full => 0,
                ImageZone::Partial(ref start, _) => start.x,
            }
        };

        'outer: for y in start_y..height {
            for x in start_x..width {
                let pixel = self.image.get_pixel(x, y);
                let rgb = Rgb::from(pixel.0);
                let color = Color::Rgb(rgb);
                let point = Point { x, y };
                let loop_result = callback(color, point);
                if let Some(result) = loop_result {
                    match result {
                        LoopResult::Continue(Axis::Y) => {
                            continue 'outer;
                        }
                        LoopResult::Continue(Axis::X) => {
                            break;
                        }
                        LoopResult::Break(Axis::Y) => {
                            break 'outer;
                        }
                        LoopResult::Break(Axis::X) => {
                            break;
                        }
                    }
                }
            }
        }
    }

    pub fn detect_pixels(&mut self, zone: ImageZone) -> &PixelVec {
        if self.pixels.points_count > 0 {
            return &self.pixels;
        }
        let mut points = PixelVec::new();
        self.process_image(zone, |color, point| {
            points.push((color, point));
            None
        });
        self.pixels = points;
        &self.pixels
    }
    #[allow(dead_code)]
    pub fn detect_pixels_color(&mut self, zone: ImageZone, color: Color) -> &PixelVec {
        if self.pixels.points_count > 0 {
            return &self.pixels;
        }
        let mut points = PixelVec::new();
        self.process_image(zone, |c, point| {
            if c == color {
                points.push((c, point));
            }
            None
        });
        self.pixels = points;
        &self.pixels
    }

    pub fn detect_pixel_with_tolerance(
        &mut self,
        zone: ImageZone,
        color: Color,
        tolerance: Color,
    ) -> &PixelVec {
        if self.pixels.points_count > 0 {
            return &self.pixels;
        }
        let mut points = PixelVec::new();
        self.process_image(zone, |c, point| {
            if c == color || c.get_hsv().compare(&color.get_hsv(), tolerance.get_hsv()) {
                points.push((c, point));
            }
            None
        });
        self.pixels = points;
        &self.pixels
    }

    #[allow(dead_code)]
    pub fn batch_zones(&self, zones: Vec<ImageZone>) -> Vec<ImageZone> {
        // merge zones that have the exact same zone
        let mut zones_result = Vec::new();
        for zone in zones {
            let existing_zone = zones_result.iter_mut().find(|z| *z == &zone);
            if let None = existing_zone {
                zones_result.push(zone);
            }
        }
        zones_result
    }
    #[allow(dead_code)]
    pub fn detect_zones<F>(&mut self, zones: Vec<ImageZone>)
    where
        F: FnMut(Color, Point) -> Option<LoopResult>,
    {
        for zone in zones {
            self.detect_pixels(zone);
        }
    }
}
