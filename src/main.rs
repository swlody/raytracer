use crate::camera::Camera;
use crate::material::Lambertian;
use crate::material::Metal;
use crate::ray::Ray;
use crate::sphere::dot;
use crate::sphere::Hitable;
use crate::sphere::Sphere;
use crate::sphere::SphereList;
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

pub fn vector_multiply(a: &Vector3<f32>, b: &Vector3<f32>) -> Vector3<f32> {
    Vector3::new(a.x * b.x, a.y * b.y, a.z * b.z)
}

pub fn unit_vector(vector: &Vector3<f32>) -> Vector3<f32> {
    return vector / dot(vector, vector).sqrt();
}

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

fn random_in_unit_sphere() -> Vector3<f32> {
    loop {
        let p = Vector3::new(
            rand::thread_rng().gen::<f32>(),
            rand::thread_rng().gen::<f32>(),
            rand::thread_rng().gen::<f32>(),
        ) * 2.0
            - Vector3::new(1.0, 1.0, 1.0);

        if dot(&p, &p) < 1.0 {
            return p;
        }
    }
}

fn main() {
    let nx: u32 = 1000;
    let ny: u32 = 500;
    let ns: u32 = 100;

    let mut buffer: Vec<u8> = Vec::with_capacity(((nx * ny) * 4) as usize);

    let camera = Camera::new(
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(-2.0, -1.0, -1.0),
        Vector3::new(4.0, 0.0, 0.0),
        Vector3::new(0.0, 2.0, 0.0),
    );

    let lam1 = Lambertian::new(Vector3::new(0.8, 0.3, 0.3));
    let lam2 = Lambertian::new(Vector3::new(0.8, 0.8, 0.0));
    let met1 = Metal::new(Vector3::new(0.8, 0.6, 0.2), 1.0);
    let met2 = Metal::new(Vector3::new(0.8, 0.8, 0.8), 0.3);

    let world = SphereList {
        list: vec![
            Sphere::new(Vector3::new(0.0, 0.0, -1.0), 0.5, &lam1),
            Sphere::new(Vector3::new(0.0, -100.5, -1.0), 100.0, &lam2),
            Sphere::new(Vector3::new(1.0, 0.0, -1.0), 0.5, &met1),
            Sphere::new(Vector3::new(-1.0, 0.0, -1.0), 0.5, &met2),
        ],
    };

    for j in (0..ny).rev() {
        for i in 0..nx {
            let column = (0..ns)
                .map(|_| {
                    let u = (i as f32 + rand::thread_rng().gen::<f32>()) / nx as f32;
                    let v = (j as f32 + rand::thread_rng().gen::<f32>()) / ny as f32;

                    let ray = camera.get_ray(u, v);

                    // let p = ray.point_at_parameter(2.0);

                    color(&ray, &world, 0)
                })
                .sum::<Vector3<f32>>()
                / ns as f32;

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
