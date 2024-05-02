use std::io;

use image::{ ImageBuffer, Rgba };
use image_analyzer::{ color::{ hsv::Hsv, rgb::Rgb, Color }, pixel::PixelVec, ImageZone };
use utils::benchmark::Benchmark;

mod data;
mod image_analyzer;
mod utils;

const BENCH_ITER: u32 = 100;
const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error: unable to read user input");
    input
}
#[allow(dead_code)]
fn prompt(message: &str) -> String {
    println!("{}", message);
    get_input()
}

fn prompt_with_default(message: &str, default: &str) -> String {
    println!("{} (default: {})", message, default);
    let input = get_input();
    if input.trim() == "" {
        default.to_string()
    } else {
        input
    }
}
fn main() {
    let bench_iter = prompt_with_default("Enter number of iterations", &BENCH_ITER.to_string())
        .trim()
        .parse::<u32>()
        .unwrap_or(BENCH_ITER);

    let bench_width = prompt_with_default("Enter image width", &WIDTH.to_string())
        .trim()
        .parse::<u32>()
        .unwrap_or(WIDTH);

    let bench_height = prompt_with_default("Enter image height", &HEIGHT.to_string())
        .trim()
        .parse::<u32>()
        .unwrap_or(HEIGHT);
    let image: ImageBuffer<Rgba<u8>, Vec<u8>> = image::ImageBuffer::new(bench_width, bench_height);
    let mut benchmark = Benchmark::new(bench_iter, "bench_detect_v2", true);
    benchmark.run(|i| {
        let mut analyzer = image_analyzer::ImageAnalyzer::new(image.clone());
        let px_vec = bench_detect_v2(&mut analyzer);
        if i == bench_iter - 1 {
            println!("Size: {}, Total: {}", px_vec.pixels.len(), px_vec.points_count);
        }
    });
    println!("------------------------------------------");
    println!("Done image size: {}x{}", bench_width, bench_height);
}

fn bench_detect_v2(analyzer: &mut image_analyzer::ImageAnalyzer) -> &PixelVec {
    //analyzer.pixel_detectv2(ImageZone::Full)

    analyzer.detect_pixel_with_tolerance(
        ImageZone::Full,
        Color::Rgb(Rgb::from([0, 0, 0, 0])),
        Color::Hsv(Hsv::from([0.5, 0.5, 0.5, 0.5]))
    )
}
