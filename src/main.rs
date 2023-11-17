mod color;

use color::Palette;
use image::{ImageBuffer, RgbImage};
use rayon::prelude::*;

struct Bounds {
    left: f32,
    right: f32,
    top: f32,
    bottom: f32,
}

const MAX_ITERS: u32 = 255;

fn mandelbrot(c: num::Complex<f32>, dist: f32, max_iters: u32) -> u32 {
    let mut z = num::Complex::new(0.0, 0.0);
    let mut n = 0;

    while z.norm() <= dist && n < max_iters {
        z = z * z + c;
        n += 1;
    }

    n
}

fn julia(c: num::Complex<f32>, dist: f32, max_iters: u32) -> u32 {
    let mut z = c;
    let mut n = 0;

    while z.norm() <= dist && n < max_iters {
        z = z * z + num::Complex::new(-0.8, 0.156);
        n += 1;
    }

    n
}

fn fractal(
    fractal_func: impl Fn(num::Complex<f32>, f32, u32) -> u32,
    dist: f32,
    max_iters: u32,
) -> impl Fn(f32, f32) -> f32 {
    move |x: f32, y: f32| -> f32 {
        let c = num::Complex::new(x, y);
        let n = fractal_func(c, dist, max_iters) as f32;
        (n / max_iters as f32).sqrt()
    }
}

fn canvas(
    bounds: Bounds,
    width: u32,
    height: u32,
    func: impl Fn(f32, f32) -> f32 + Send + Sync,
) -> Vec<f32> {
    let scalex = (bounds.right - bounds.left) / width as f32;
    let scaley = (bounds.top - bounds.bottom) / height as f32;

    (0..width * height)
        .into_par_iter()
        .map(|i| {
            let x = i % width;
            let y = i / width;
            let cx = x as f32 * scalex + bounds.left;
            let cy = y as f32 * scaley + bounds.bottom;
            func(cx, cy)
        })
        .collect()
}

fn draw(data: Vec<f32>, width: u32, height: u32, palette: Palette) -> RgbImage {
    let mut img: RgbImage = ImageBuffer::new(width, height);

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let value = data[(y * width + x) as usize];
        *pixel = image::Rgb(palette.color(value));
    }

    img
}

fn draw_nearest(data: Vec<f32>, width: u32, height: u32, palette: Palette) -> RgbImage {
    let mut img: RgbImage = ImageBuffer::new(width, height);

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let value = data[(y * width + x) as usize];
        *pixel = image::Rgb(palette.nearest_color(value));
    }

    img
}

const HD: (u32, u32) = (1280, 720);
const FHD: (u32, u32) = (1920, 1080);
const QHD: (u32, u32) = (2560, 1440);
const UHD: (u32, u32) = (3840, 2160);

fn main() {
    // let (width, height) = UHD;
    let (width, height) = (1000, 1000);

    let bounds = Bounds {
        left: -2.0,
        right: 0.5,
        top: 1.125,
        bottom: -1.125,
    };

    let func = fractal(mandelbrot, 4.0, MAX_ITERS);
    let data = canvas(bounds, width, height, func);

    println!("Generated mandelbrot set");

    let img = draw(data, width, height, Palette::wiki());

    img.save("mandelbrot.png").unwrap();
    println!("Mandelbrot set generated and saved as 'mandelbrot.png'");
}
