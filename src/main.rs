use image::{ImageBuffer, Rgba};
use image_analyzer::{
    color::{hsv::Hsv, rgb::Rgb, Color},
    pixel::PixelVec,
    ImageZone,
};
use utils::benchmark::Benchmark;

mod data;
mod image_analyzer;
mod utils;

const BENCH_ITER: u32 = 100;
const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;
fn main() {
    let image: ImageBuffer<Rgba<u8>, Vec<u8>> = image::ImageBuffer::new(WIDTH, HEIGHT);
    let mut benchmark = Benchmark::new(BENCH_ITER, "bench_detect_v2", true);
    benchmark.run(|i| {
        let mut analyzer = image_analyzer::ImageAnalyzer::new(image.clone());
        let px_vec = bench_detect_v2(&mut analyzer);
        if i == BENCH_ITER - 1 {
            println!(
                "Size: {}, Total: {}",
                px_vec.pixels.len(),
                px_vec.points_count
            );
        }
    });
    println!("------------------------------------------");
    println!("Done image size: {}x{}", WIDTH, HEIGHT);
}

fn bench_detect_v2(analyzer: &mut image_analyzer::ImageAnalyzer) -> &PixelVec {
    //analyzer.pixel_detectv2(ImageZone::Full)

    analyzer.detect_pixel_with_tolerance(
        ImageZone::Full,
        Color::Rgb(Rgb::from([0, 0, 0, 0])),
        Color::Hsv(Hsv::from([0.5, 0.5, 0.5, 0.5])),
    )
}
