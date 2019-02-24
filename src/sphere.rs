use crate::ray::Ray;
use cgmath::Vector3;

pub struct HitRecord {
    pub t: f32,
    pub p: Vector3<f32>,
    pub normal: Vector3<f32>,
}

pub trait Hitable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

pub struct Sphere {
    pub center: Vector3<f32>,
    pub radius: f32,
}

pub fn dot(a: &Vector3<f32>, b: &Vector3<f32>) -> f32 {
    a.x * b.x + a.y * b.y + a.z * b.z
}

impl Hitable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = dot(&ray.direction, &ray.direction);
        let b = dot(&oc, &ray.direction);
        let c = dot(&oc, &oc) - (self.radius * self.radius);
        let discriminant = (b * b) - (a * c);

        if discriminant > 0.0 {
            let temp = (-b - (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                let p = ray.point_at_parameter(temp);

                return Some(HitRecord {
                    t: temp,
                    p: p,
                    normal: (p - self.center) / self.radius,
                });
            }
        }

        None
    }
}

pub struct SphereList {
    pub list: Vec<Sphere>,
}

impl Hitable for SphereList {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut record: Option<HitRecord> = None;

        for sphere in &self.list {
            if let Some(temp_record) = sphere.hit(ray, t_min, closest_so_far) {
                closest_so_far = temp_record.t;
                record = Some(temp_record);
            }
        }

        record
    }
}
