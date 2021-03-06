use crate::material::Material;
use crate::ray::Ray;
use crate::vector_utils::*;
use cgmath::Vector3;
use rand::Rng;

pub fn random_in_unit_sphere() -> Vector3<f32> {
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

pub struct HitRecord<'a> {
    pub t: f32,
    pub p: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub material: &'a Material,
}

impl<'a> HitRecord<'a> {
    pub fn new(t: f32, p: Vector3<f32>, normal: Vector3<f32>, material: &'a Material) -> HitRecord {
        HitRecord {
            t: t,
            p: p,
            normal: normal,
            material: material,
        }
    }
}

pub trait Hitable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

pub struct Sphere {
    pub center: Vector3<f32>,
    pub radius: f32,
    pub material: Material,
}

impl Sphere {
    pub fn new(center: Vector3<f32>, radius: f32, material: Material) -> Sphere {
        Sphere {
            center: center,
            radius: radius,
            material: material,
        }
    }
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

                return Some(HitRecord::new(
                    temp,
                    p,
                    (p - self.center) / self.radius,
                    &self.material,
                ));
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
