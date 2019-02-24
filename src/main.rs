use crate::camera::Camera;
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
mod ray;
mod sphere;

pub fn unit_vector(vector: &Vector3<f32>) -> Vector3<f32> {
    return vector / dot(vector, vector).sqrt();
}

fn color(ray: &Ray, world: &Hitable) -> Vector3<f32> {
    match world.hit(ray, 0.001, std::f32::MAX) {
        Some(record) => {
            let target = record.p + record.normal + random_in_unit_sphere();

            0.5 * color(
                &Ray {
                    origin: record.p,
                    direction: target - record.p,
                },
                world,
            )
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

    let camera = Camera {
        lower_left_corner: Vector3::new(-2.0, -1.0, -1.0),
        horizontal: Vector3::new(4.0, 0.0, 0.0),
        vertical: Vector3::new(0.0, 2.0, 0.0),
        origin: Vector3::new(0.0, 0.0, 0.0),
    };

    let world = SphereList {
        list: vec![
            Sphere {
                center: Vector3::new(0.0, 0.0, -1.0),
                radius: 0.5,
            },
            Sphere {
                center: Vector3::new(0.0, -100.5, -1.0),
                radius: 100.0,
            },
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

                    color(&ray, &world)
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
