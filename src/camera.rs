use crate::ray::Ray;
use crate::unit_vector;
use crate::vector_utils::cross;
use crate::vector_utils::dot;
use cgmath::Vector3;
use rand::Rng;

fn random_in_unit_disk() -> Vector3<f32> {
    loop {
        let p =
            2.0 * Vector3::new(
                rand::thread_rng().gen::<f32>(),
                rand::thread_rng().gen::<f32>(),
                0.0,
            ) - Vector3::new(1.0, 1.0, 0.0);
        if dot(&p, &p) < 1.0 {
            return p;
        }
    }
}

pub struct Camera {
    origin: Vector3<f32>,
    lower_left_corner: Vector3<f32>,
    horizontal: Vector3<f32>,
    vertical: Vector3<f32>,

    u: Vector3<f32>,
    v: Vector3<f32>,
    lens_radius: f32,
}

impl Camera {
    pub fn new(
        lookfrom: Vector3<f32>,
        lookat: Vector3<f32>,
        vup: Vector3<f32>,
        vfov: f32,
        aspect: f32,
        aperture: f32,
        focus_dist: f32,
    ) -> Camera {
        let theta = vfov.to_radians();
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;

        let w = unit_vector(&(lookfrom - lookat));
        let u = unit_vector(&cross(&vup, &w));
        let v = cross(&w, &u);

        Camera {
            origin: lookfrom,
            lower_left_corner: lookfrom
                - half_width * focus_dist * u
                - half_height * focus_dist * v
                - focus_dist * w,
            horizontal: 2.0 * half_width * focus_dist * u,
            vertical: 2.0 * half_height * focus_dist * v,
            u: u,
            v: v,
            lens_radius: aperture / 2.0,
        }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
        )
    }
}
