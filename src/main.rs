use cgmath::Vector3;
use png::{Encoder, HasParameters};
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

struct Ray {
    origin: Vector3<f32>,
    direction: Vector3<f32>,
}

impl Ray {
    fn point_at_parameter(&self, t: f32) -> Vector3<f32> {
        return self.origin + (t * self.direction);
    }
}

fn dot(a: &Vector3<f32>, b: &Vector3<f32>) -> f32 {
    a.x * b.x + a.y * b.y + a.z * b.z
}

fn unit_vector(vector: &Vector3<f32>) -> Vector3<f32> {
    return vector / dot(vector, vector).sqrt();
}

fn hit_sphere(center: &Vector3<f32>, radius: f32, ray: &Ray) -> f32 {
    let oc = ray.origin - center;
    let a = dot(&ray.direction, &ray.direction);
    let b = 2.0 * dot(&oc, &ray.direction);
    let c = dot(&oc, &oc) - (radius * radius);

    let discriminant = (b * b) - (4.0 * a * c);

    if discriminant < 0.0 {
        -1.0
    } else {
        (-b - discriminant.sqrt()) / (2.0 * a)
    }
}

fn color(ray: &Ray) -> Vector3<f32> {
    let t = hit_sphere(&Vector3::new(0.0, 0.0, -1.0), 0.5, ray);

    if t > 0.0 {
        let n = unit_vector(&(ray.point_at_parameter(t) - Vector3::new(0.0, 0.0, -1.0)));

        return 0.5 * Vector3::new(n.x + 1.0, n.y + 1.0, n.z + 1.0);
    }

    let unit_direction = unit_vector(&ray.direction);
    let t = 0.5 * (unit_direction.y + 1.0);

    (1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0)
}

fn main() {
    let nx: u32 = 1000;
    let ny: u32 = 500;

    let mut buffer: Vec<u8> = Vec::with_capacity(((nx * ny) * 4) as usize);

    let lower_left_corner = Vector3::new(-2.0, -1.0, -1.0);
    let horizontal = Vector3::new(4.0, 0.0, 0.0);
    let vertical = Vector3::new(0.0, 2.0, 0.0);
    let origin = Vector3::new(0.0, 0.0, 0.0);

    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = i as f32 / nx as f32;
            let v = j as f32 / ny as f32;

            let ray = Ray {
                origin: origin,
                direction: lower_left_corner + u * horizontal + v * vertical,
            };

            let col = color(&ray);

            let ir = (255.99 * col[0]) as u8;
            let ig = (255.99 * col[1]) as u8;
            let ib = (255.99 * col[2]) as u8;

            buffer.extend([ir, ig, ib, 255].iter().cloned());
        }
    }

    write_png_to_file("render.png", &buffer[..], nx, ny);
}

fn write_png_to_file(filename: &str, data: &[u8], width: u32, height: u32) {
    let path = Path::new(filename);
    let file = File::create(&path).unwrap();

    let ref mut writer = BufWriter::new(file);
    let mut encoder = Encoder::new(writer, width, height);
    encoder.set(png::ColorType::RGBA).set(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();

    writer.write_image_data(data).unwrap();
}
