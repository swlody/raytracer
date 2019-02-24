use crate::camera::Camera;
use crate::material::Dielectric;
use crate::material::Lambertian;
use crate::material::Metal;
use crate::ray::Ray;
use crate::sphere::Hitable;
use crate::sphere::Sphere;
use crate::sphere::SphereList;
use crate::vector_utils::unit_vector;
use crate::vector_utils::vector_multiply;
use cgmath::Vector3;
use png::{Encoder, HasParameters};
use rand::Rng;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

mod camera;
mod material;
mod ray;
mod sphere;
mod vector_utils;

fn color(ray: &Ray, world: &Hitable, depth: i32) -> Vector3<f32> {
    match world.hit(ray, 0.001, std::f32::MAX) {
        Some(record) => {
            if depth < 50 {
                if let Some((scattered, attenuation)) = record.material.scatter(&ray, &record) {
                    return vector_multiply(&attenuation, &color(&scattered, world, depth + 1));
                }
            }

            Vector3::new(0.0, 0.0, 0.0)
        }
        None => {
            let unit_direction = unit_vector(&ray.direction);
            let t = 0.5 * (unit_direction.y + 1.0);
            (1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0)
        }
    }
}

fn main() {
    let nx: u32 = 1000;
    let ny: u32 = 500;
    let ns: u32 = 100;

    let mut buffer: Vec<u8> = Vec::with_capacity(((nx * ny) * 4) as usize);

    let camera = Camera::new(
        Vector3::new(-2.0, 2.0, 1.0),
        Vector3::new(0.0, 0.0, -1.0),
        Vector3::new(0.0, 1.0, 0.0),
        90.0,
        nx as f32 / ny as f32,
    );

    let lam1 = Lambertian::new(Vector3::new(0.1, 0.2, 0.5));
    let lam2 = Lambertian::new(Vector3::new(0.8, 0.8, 0.0));
    let met1 = Metal::new(Vector3::new(0.8, 0.6, 0.2), 0.0);
    let die1 = Dielectric::new(1.5);

    let world = SphereList {
        list: vec![
            Sphere::new(Vector3::new(0.0, 0.0, -1.0), 0.5, &lam1),
            Sphere::new(Vector3::new(0.0, -100.5, -1.0), 100.0, &lam2),
            Sphere::new(Vector3::new(1.0, 0.0, -1.0), 0.5, &met1),
            Sphere::new(Vector3::new(-1.0, 0.0, -1.0), 0.5, &die1),
            Sphere::new(Vector3::new(-1.0, 0.0, -1.0), -0.45, &die1),
        ],
    };

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut column = Vector3::new(0.0, 0.0, 0.0);

            for _ in 0..ns {
                let u = (i as f32 + rand::thread_rng().gen::<f32>()) / nx as f32;
                let v = (j as f32 + rand::thread_rng().gen::<f32>()) / ny as f32;

                let ray = camera.get_ray(u, v);

                column += color(&ray, &world, 0);
            }

            column /= ns as f32;

            let ir = (255.99 * column[0].sqrt()) as u8;
            let ig = (255.99 * column[1].sqrt()) as u8;
            let ib = (255.99 * column[2].sqrt()) as u8;

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
