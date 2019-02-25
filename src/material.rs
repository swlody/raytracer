use crate::ray::Ray;
use crate::sphere::random_in_unit_sphere;
use crate::sphere::HitRecord;
use crate::unit_vector;
use crate::vector_utils::dot;
use crate::vector_utils::vector_length;
use cgmath::Vector3;
use rand::Rng;

pub trait Scatterable {
    fn scatter(&self, ray_in: &Ray, record: &HitRecord) -> Option<(Ray, Vector3<f32>)>;
}

pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
}

impl Scatterable for Material {
    fn scatter(&self, ray_in: &Ray, record: &HitRecord) -> Option<(Ray, Vector3<f32>)> {
        match *self {
            Material::Lambertian(ref inner) => inner.scatter(ray_in, record),
            Material::Metal(ref inner) => inner.scatter(ray_in, record),
            Material::Dielectric(ref inner) => inner.scatter(ray_in, record),
        }
    }
}

pub struct Lambertian {
    pub albedo: Vector3<f32>,
}

impl Lambertian {
    pub fn new(albedo_x: f32, albedo_y: f32, albedo_z: f32) -> Lambertian {
        Lambertian {
            albedo: Vector3::new(albedo_x, albedo_y, albedo_z),
        }
    }
}

impl Scatterable for Lambertian {
    #[allow(unused)]
    fn scatter(&self, ray_in: &Ray, record: &HitRecord) -> Option<(Ray, Vector3<f32>)> {
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
    pub fn new(albedo_x: f32, albedo_y: f32, albedo_z: f32, fuzz: f32) -> Metal {
        Metal {
            albedo: Vector3::new(albedo_x, albedo_y, albedo_z),
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}

impl Scatterable for Metal {
    fn scatter(&self, ray_in: &Ray, record: &HitRecord) -> Option<(Ray, Vector3<f32>)> {
        let reflected = reflect(&unit_vector(&ray_in.direction), &record.normal);
        let scattered = Ray::new(record.p, reflected + self.fuzz * random_in_unit_sphere());

        if dot(&scattered.direction, &record.normal) > 0.0 {
            Some((scattered, self.albedo))
        } else {
            None
        }
    }
}

pub struct Dielectric {
    ref_idx: f32,
}

impl Dielectric {
    pub fn new(ref_idx: f32) -> Dielectric {
        Dielectric { ref_idx: ref_idx }
    }
}

fn refract(v: &Vector3<f32>, n: &Vector3<f32>, ni_over_nt: f32) -> Option<Vector3<f32>> {
    let uv = unit_vector(&v);
    let dt = dot(&uv, &n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);

    if discriminant > 0.0 {
        Some(ni_over_nt * (uv - n * dt) - n * discriminant.sqrt())
    } else {
        None
    }
}

fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 * r0 + (1.0 - r0 * r0) * (1.0 - cosine).powi(5)
}

impl Scatterable for Dielectric {
    fn scatter(&self, ray_in: &Ray, record: &HitRecord) -> Option<(Ray, Vector3<f32>)> {
        let dir = dot(&ray_in.direction, &record.normal);
        let (outward_normal, ni_over_nt, cosine) = if dir > 0.0 {
            (
                -record.normal,
                self.ref_idx,
                self.ref_idx * dir / vector_length(&ray_in.direction),
            )
        } else {
            (
                record.normal,
                1.0 / self.ref_idx,
                -dir / vector_length(&ray_in.direction),
            )
        };

        if let Some(refracted) = refract(&ray_in.direction, &outward_normal, ni_over_nt) {
            if rand::thread_rng().gen::<f32>() >= schlick(cosine, self.ref_idx) {
                return Some((Ray::new(record.p, refracted), Vector3::new(1.0, 1.0, 1.0)));
            }
        }
        Some((
            Ray::new(record.p, reflect(&ray_in.direction, &record.normal)),
            Vector3::new(1.0, 1.0, 1.0),
        ))
    }
}

pub fn reflect(v: &Vector3<f32>, n: &Vector3<f32>) -> Vector3<f32> {
    v - 2.0 * dot(v, n) * n
}
