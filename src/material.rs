use crate::random_in_unit_sphere;
use crate::ray::Ray;
use crate::sphere::dot;
use crate::sphere::HitRecord;
use crate::unit_vector;
use cgmath::Vector3;

pub trait Material {
    fn scatter(&self, r_in: &Ray, record: &HitRecord) -> Option<(Ray, Vector3<f32>)>;
}

pub struct Lambertian {
    pub albedo: Vector3<f32>,
}

impl Lambertian {
    pub fn new(albedo: Vector3<f32>) -> Lambertian {
        Lambertian { albedo: albedo }
    }
}

impl Material for Lambertian {
    #[allow(unused)]
    fn scatter(&self, r_in: &Ray, record: &HitRecord) -> Option<(Ray, Vector3<f32>)> {
        Some((
            Ray::new(record.p, record.normal + random_in_unit_sphere()),
            self.albedo,
        ))
    }
}

pub struct Metal {
    pub albedo: Vector3<f32>,
    pub fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Vector3<f32>, fuzz: f32) -> Metal {
        Metal {
            albedo: albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, record: &HitRecord) -> Option<(Ray, Vector3<f32>)> {
        let reflected = reflect(&unit_vector(&r_in.direction), &record.normal);
        let scattered = Ray::new(record.p, reflected + self.fuzz * random_in_unit_sphere());

        if dot(&scattered.direction, &record.normal) > 0.0 {
            Some((scattered, self.albedo))
        } else {
            None
        }
    }
}

pub fn reflect(v: &Vector3<f32>, n: &Vector3<f32>) -> Vector3<f32> {
    v - 2.0 * dot(v, n) * n
}
