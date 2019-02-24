use png::HasParameters;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

fn main() {
    let path = Path::new("render.png");

    let file = File::create(&path).unwrap();

    let nx = 1000;
    let ny = 500;

    let mut vec: Vec<u8> = Vec::new();

    for j in (0..ny).rev() {
        for i in 0..nx {
            let r = i as f32 / nx as f32;
            let g = j as f32 / ny as f32;
            let b = 0.2;

            let ir = (255.99 * r) as u8;
            let ig = (255.99 * g) as u8;
            let ib = (255.99 * b) as u8;

            vec.extend([ir, ig, ib, 255].iter().cloned());
        }
    }

    let ref mut writer = BufWriter::new(file);

    let mut encoder = png::Encoder::new(writer, nx, ny);
    encoder.set(png::ColorType::RGBA).set(png::BitDepth::Eight);

    let mut writer = encoder.write_header().unwrap();

    writer.write_image_data(&vec[..]).unwrap();
}
