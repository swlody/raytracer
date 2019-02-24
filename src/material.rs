use crate::random_in_unit_sphere;
use crate::ray::Ray;
use crate::sphere::dot;
use crate::sphere::HitRecord;
use crate::unit_vector;
use cgmath::Vector3;
use rand::Rng;

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

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, record: &HitRecord) -> Option<(Ray, Vector3<f32>)> {
        let (outward_normal, ni_over_nt, cosine) = if dot(&r_in.direction, &record.normal) > 0.0 {
            (
                -record.normal,
                self.ref_idx,
                self.ref_idx * dot(&r_in.direction, &record.normal)
                    / dot(&r_in.direction, &r_in.direction).sqrt(),
            )
        } else {
            (
                record.normal,
                1.0 / self.ref_idx,
                -dot(&r_in.direction, &record.normal)
                    / dot(&r_in.direction, &r_in.direction).sqrt(),
            )
        };

        if let Some(refracted) = refract(&r_in.direction, &outward_normal, ni_over_nt) {
            if rand::thread_rng().gen::<f32>() >= schlick(cosine, self.ref_idx) {
                return Some((Ray::new(record.p, refracted), Vector3::new(1.0, 1.0, 1.0)));
            }
        }
        Some((
            Ray::new(record.p, reflect(&r_in.direction, &record.normal)),
            Vector3::new(1.0, 1.0, 1.0),
        ))
    }
}

pub fn reflect(v: &Vector3<f32>, n: &Vector3<f32>) -> Vector3<f32> {
    v - 2.0 * dot(v, n) * n
}
