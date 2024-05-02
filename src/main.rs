use data::better_call_zone::Zone;
use image::{ ImageBuffer, Rgba };
use image_analyzer::{ pixel::{ Pixel, PixelVec, _PixelVec }, ImageZone };
use rayon::vec;
use utils::benchmark::Benchmark;

mod utils;
mod data;
mod image_analyzer;

const BENCH_ITER: u32 = 100;
const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;
fn main() {
    let image: ImageBuffer<Rgba<u8>, Vec<u8>> = image::ImageBuffer::new(WIDTH, HEIGHT);
    
    let mut benchmark = Benchmark::new(BENCH_ITER, "bench_closure_par_detect_v1");
    benchmark.run(|i| { 
        let px_vec = bench_closure_par_detect_v1(image.clone());
        if i == BENCH_ITER - 1 {
            println!("Size: {}, Total: {}", px_vec.pixels.len(), px_vec.points_count);
        }
    });
    benchmark.print();
    println!("------------------------------------------");

    let mut benchmark = Benchmark::new(BENCH_ITER, "bench_closure_detect_v1");
    benchmark.run(|i| { 
        let px_vec = bench_closure_detect_v1(image.clone());
        if i == BENCH_ITER - 1 {
            println!("Size: {}, Total: {}", px_vec.pixels.len(), px_vec.points_count);
        }

    });
    benchmark.print();
    println!("------------------------------------------");

    let mut benchmark = Benchmark::new(BENCH_ITER, "bench_closure_detect_v2");
    benchmark.run(|i| { 
        let px_vec = bench_closure_detect_v2(image.clone());
        if i == BENCH_ITER - 1 {
            println!("Size: {}, Total: {}", px_vec.pixels.len(), px_vec.points_count);
        }

    });
    benchmark.print();
    println!("------------------------------------------");
    println!("Done image size: {}x{}", WIDTH, HEIGHT);
}

fn bench_closure_detect_v1(image: ImageBuffer<Rgba<u8>, Vec<u8>>) -> _PixelVec {
    let mut points = _PixelVec::new();
    let mut analyzer = image_analyzer::ImageAnalyzer::new(image);
    analyzer.pixel_detectv1(|color, point| {
        points.push((color, point));
        None
    });
    points
}

fn bench_closure_detect_v2(image: ImageBuffer<Rgba<u8>, Vec<u8>>) -> _PixelVec {
    let mut points = _PixelVec::new();
    let mut analyzer = image_analyzer::ImageAnalyzer::new(image);
    analyzer.pixel_detectv2(ImageZone::Full, |color, point| {
        points.push((color, point));
        None
    });
    points
}

fn bench_closure_par_detect_v1(image: ImageBuffer<Rgba<u8>, Vec<u8>>)  -> _PixelVec {
    let mut analyzer = image_analyzer::ImageAnalyzer::new(image);
    let res = analyzer.par_pixel_detectv1(|color, point| {
        None
    });
    res
}
