use crate::camera::Camera;
use crate::material::Dielectric;
use crate::material::Lambertian;
use crate::material::Material;
use crate::material::Metal;
use crate::material::Scatterable;
use crate::ray::Ray;
use crate::sphere::Hitable;
use crate::sphere::Sphere;
use crate::sphere::SphereList;
use crate::vector_utils::unit_vector;
use crate::vector_utils::vector_length;
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
    let nx: u32 = 1280;
    let ny: u32 = 720;
    let ns: u32 = 100;

    let mut buffer: Vec<u8> = Vec::with_capacity(((nx * ny) * 4) as usize);

    let lookfrom = Vector3::new(15.0, 3.0, 2.0);
    let lookat = Vector3::new(0.0, 0.0, 0.0);

    let camera = Camera::new(
        lookfrom,
        lookat,
        Vector3::new(0.0, 1.0, 0.0),
        20.0,
        nx as f32 / ny as f32,
        0.1,
        vector_length(&(lookfrom - lookat)),
    );

    let world = random_scene();

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

fn random_scene() -> SphereList {
    let mut world = SphereList {
        list: Vec::with_capacity(500 + 1),
    };

    world.list.push(Sphere::new(
        Vector3::new(0.0, -1000.0, 0.0),
        1000.0,
        Material::Lambertian(Lambertian::new(0.5, 0.5, 0.5)),
    ));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rand::thread_rng().gen::<f32>();
            let center = Vector3::new(
                a as f32 + 0.9 + rand::thread_rng().gen::<f32>(),
                0.2,
                b as f32 + 0.9 * rand::thread_rng().gen::<f32>(),
            );

            if vector_length(&(center - Vector3::new(4.0, 0.2, 0.0))) > 0.9 {
                if choose_mat < 0.8 {
                    world.list.push(Sphere::new(
                        center,
                        0.2,
                        Material::Lambertian(Lambertian::new(
                            rand::thread_rng().gen::<f32>() * rand::thread_rng().gen::<f32>(),
                            rand::thread_rng().gen::<f32>() * rand::thread_rng().gen::<f32>(),
                            rand::thread_rng().gen::<f32>() * rand::thread_rng().gen::<f32>(),
                        )),
                    ));
                } else if choose_mat < 0.95 {
                    world.list.push(Sphere::new(
                        center,
                        0.2,
                        Material::Metal(Metal::new(
                            0.5 * (1.0 + rand::thread_rng().gen::<f32>()),
                            0.5 * (1.0 + rand::thread_rng().gen::<f32>()),
                            0.5 * (1.0 + rand::thread_rng().gen::<f32>()),
                            0.5 * rand::thread_rng().gen::<f32>(),
                        )),
                    ));
                } else {
                    world.list.push(Sphere::new(
                        center,
                        0.2,
                        Material::Dielectric(Dielectric::new(1.5)),
                    ));
                }
            }
        }
    }

    world.list.push(Sphere::new(
        Vector3::new(0.0, 1.0, 0.0),
        1.0,
        Material::Dielectric(Dielectric::new(1.5)),
    ));
    world.list.push(Sphere::new(
        Vector3::new(-4.0, 1.0, 0.0),
        1.0,
        Material::Lambertian(Lambertian::new(0.4, 0.2, 0.1)),
    ));
    world.list.push(Sphere::new(
        Vector3::new(4.0, 1.0, 0.0),
        1.0,
        Material::Metal(Metal::new(0.7, 0.6, 0.5, 0.0)),
    ));

    world
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
