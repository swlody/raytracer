use crate::ray::Ray;
use cgmath::Vector3;

pub struct Camera {
    pub origin: Vector3<f32>,
    pub lower_left_corner: Vector3<f32>,
    pub horizontal: Vector3<f32>,
    pub vertical: Vector3<f32>,
}

impl Camera {
    pub fn new(
        origin: Vector3<f32>,
        lower_left_corner: Vector3<f32>,
        horizontal: Vector3<f32>,
        vertical: Vector3<f32>,
    ) -> Camera {
        Camera {
            origin: origin,
            lower_left_corner: lower_left_corner,
            horizontal: horizontal,
            vertical: vertical,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.lower_left_corner + u * self.horizontal + v * self.vertical
                - self.origin,
        }
    }
}
