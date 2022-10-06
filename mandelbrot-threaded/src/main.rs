use num::Complex;
use std::str::FromStr;
use image::ImageEncoder;
use image::ColorType;
use image::codecs::png::PngEncoder;
use std::fs::File;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 5 {
        eprintln!("Usage: {} FILE PIXELS UPPERLEFT LOWERRIGHT", args[0]);
        eprintln!("Example: {} mandel.png 4000x3000 -1.20,0.35 -1,0.20", args[0]);

        std::process::exit(1);
    }

    let bounds = parse_pair(&args[2], 'x').expect("Error parsing image dimensions");
    let upper_left = parse_complex(&args[3]).expect("Error parsing upper left corner point");
    let lower_right = parse_complex(&args[4]).expect("Error parsing lower right corner point");

    let threads = 4;
    let mut rows_per_band = bounds.1 / threads;
    if bounds.1 % threads != 0 {
        rows_per_band += 1;
    }

    let mut pixels = vec![0; bounds.0 * bounds.1];
    
    let bands: Vec<&mut [u8]> = pixels.chunks_mut(rows_per_band * bounds.0).collect();
    crossbeam::scope(|spawner| {
        for (i, band) in bands.into_iter().enumerate() {
            let top = rows_per_band * i;
            let height = band.len() / bounds.0;
            let band_bounds = (bounds.0, height);
            let band_upper_left = pixel_to_point(bounds, (0, top), upper_left, lower_right);
            let band_lower_right = pixel_to_point(bounds, (bounds.0, top + height), upper_left, lower_right);
            spawner.spawn(move |_| {
                render(band, band_bounds, band_upper_left, band_lower_right);
            });
        }
    }).unwrap();
    write_image(&args[1], &pixels, bounds).expect("Error writing PNG file");
}

// bounds: (width, height), (real, imaginary), (x, y)
fn render(pixels: &mut [u8], bounds: (usize, usize), 
          upper_left: Complex<f64>, lower_right: Complex<f64>) {
    assert!(pixels.len() == bounds.0 * bounds.1);
    println!("Rendering...");
    for row in 0 .. bounds.1 {
        for col in 0 .. bounds.0 {
            let point = pixel_to_point(bounds, (col, row), upper_left, lower_right);
            pixels[row * bounds.0 + col] = 
                match escape_time(point, 255) {
                    Some(count) => 255 - count as u8,
                    None => 0
                }
        }
    }
    println!("Rendering done");
}

fn write_image(filename: &str, pixels: &[u8], bounds: (usize, usize)) -> Result<(), std::io::Error> {
    println!("Drawing...");

    let output = File::create(filename)?;

    let encoder = PngEncoder::new(&output);
    match encoder.write_image(
        pixels,
        bounds.0 as u32, bounds.1 as u32,
        ColorType::L8
    ) {
        Ok(_) => (),
        Err(e) => println!("{}", e)
    }
    println!("Drawing done");
    Ok(())
}

// bounds: (width, height), (real, imaginary), (x, y)
fn pixel_to_point(bounds: (usize, usize), pixel: (usize, usize), 
                  upper_left: Complex<f64>, lower_right: Complex<f64>) -> Complex<f64> {
    let (width, height) = (lower_right.re - upper_left.re, upper_left.im - lower_right.im);
    Complex {
        re: upper_left.re + pixel.0 as f64 / bounds.0 as f64 * width,
        im: upper_left.im - pixel.1 as f64 / bounds.1 as f64 * height,
    }
}

fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    
    for i in 0 .. limit {
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
        z = z * z + c;
    }

    None
}

fn parse_complex(s: &str) -> Option<Complex<f64>> {
    match parse_pair(s, ',') {
        Some((re, im)) => Some(Complex { re, im }),
        None => None
    }
}

fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        Some(index) => {
            match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
                (Ok(l), Ok(r)) => Some((l, r)),
                _ => None
            }
        },
        None => None
    }
}

#[test]
fn test_pixel_to_point() {
    assert_eq!(
        pixel_to_point((100, 200), (25, 175), Complex { re: -1.0, im: 1.0 }, Complex { re: 1.0, im: -1.0 }), 
        Complex { re: -0.5, im: -0.75 }
    );
}

#[test]
fn test_parse_complex() {
    assert_eq!(parse_complex("1.25,-0.0625"), Some(Complex {re: 1.25, im: -0.0625}));
    assert_eq!(parse_complex("-0.0625"), None);
}

#[test]
fn test_parse_pair() { 
    assert_eq!(parse_pair::<i32>("", ','), None);
    assert_eq!(parse_pair::<i32>("10,", ','), None);
    assert_eq!(parse_pair::<i32>(",10", ','), None);
    assert_eq!(parse_pair::<i32>("10,20", ','), Some((10, 20)));
    assert_eq!(parse_pair::<i32>("10,20xy", ','), None);
    assert_eq!(parse_pair::<f32>("0.5x", 'x'), None);
    assert_eq!(parse_pair::<f32>("0.5x1.5", 'x'), Some((0.5, 1.5)));
}
