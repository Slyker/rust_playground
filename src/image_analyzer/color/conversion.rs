use super::hsv::Hsv;
use super::rgb::Rgb;

pub fn hsv_from_rgb(rgb: &Rgb) -> Hsv {
    let r = rgb.r as f64 / 255.0;
    let g = rgb.g as f64 / 255.0;
    let b = rgb.b as f64 / 255.0;
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let delta = max - min;
    let h = if delta == 0.0 {
        0.0
    } else if max == r {
        60.0 * (((g - b) / delta) % 6.0)
    } else if max == g {
        60.0 * (((b - r) / delta) + 2.0)
    } else {
        60.0 * (((r - g) / delta) + 4.0)
    };
    let s = if max == 0.0 { 0.0 } else { delta / max };
    let v = max;
    Hsv {
        h,
        s,
        v,
        a: rgb.a as f64 / 255.0,
    }
}

pub fn rgb_from_hsv(hsv: &Hsv) -> Rgb {
    let c = hsv.v * hsv.s;
    let x = c * (1.0 - ((hsv.h / 60.0) % 2.0 - 1.0).abs());
    let m = hsv.v - c;
    let (r, g, b) = if hsv.h < 60.0 {
        (c, x, 0.0)
    } else if hsv.h < 120.0 {
        (x, c, 0.0)
    } else if hsv.h < 180.0 {
        (0.0, c, x)
    } else if hsv.h < 240.0 {
        (0.0, x, c)
    } else if hsv.h < 300.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };
    Rgb {
        r: ((r + m) * 255.0) as u8,
        g: ((g + m) * 255.0) as u8,
        b: ((b + m) * 255.0) as u8,
        a: (hsv.a * 255.0) as u8,
    }
}
