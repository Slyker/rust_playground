use image::ImageBuffer;
use rayon::iter::{ParallelBridge, ParallelIterator};

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
}

impl ImageAnalyzer {
    pub fn new(image: ImageBuffer<image::Rgba<u8>, Vec<u8>>) -> Self {
        Self { image }
    }

    pub fn par_pixel_detectv1(&self) -> PixelVec {
        let (snd, rcv) = std::sync::mpsc::channel();
        let image = self.image.clone();
        let _ = std::thread::spawn(move || {
            /* for (y, row) in image.enumerate_rows() {
                for (_, (x, y, pixel)) in row.enumerate() {
                    let rgb = Rgb::from(pixel.0);
                    let _ = snd.send((rgb, vec![Point { x, y }]));
                }
            } */
            image
                .enumerate_rows()
                .par_bridge()
                .for_each(move |(_, row)| {
                    for (_, (x, y, pixel)) in row.enumerate() {
                        let rgb = Rgb::from(pixel.0);
                        let _ = snd.send((rgb, Point { x, y }));
                    }
                });
        });
        let mut rgb_pixels: PixelVec = PixelVec::new();
        while let Ok((rgb, point)) = rcv.recv() {
            let rgb = Color::Rgb(rgb);
            rgb_pixels.push((rgb, point));
        }

        rgb_pixels
    }

    pub fn pixel_detectv1<F>(&self, mut callback: F)
    where
        F: FnMut(Color, Point) -> Option<LoopResult>,
    {
        'outer: for (_, row) in self.image.enumerate_rows() {
            for (_, (x, y, pixel)) in row.enumerate() {
                let rgb = Rgb::from(pixel.0);
                let loop_result = callback(Color::Rgb(rgb), Point { x, y });
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

    pub fn pixel_detectv2<F>(&self, zone: ImageZone, mut callback: F)
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
                let loop_result = callback(Color::Rgb(rgb), Point { x, y });
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
    pub fn detect_zones<F>(&self, zones: Vec<ImageZone>, mut callback: F)
    where
        F: FnMut(Color, Point) -> Option<LoopResult>,
    {
        for zone in zones {
            self.pixel_detectv2(zone, &mut callback);
        }
    }
}
