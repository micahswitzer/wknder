use crate::{ray::Ray, hittable::HitRecord, vec3::Vec3};
use rand::Rng;

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Vec3)>;
}

pub struct Lambertian(pub Vec3);

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Vec3)> {
        let target = rec.p + rec.normal + Vec3::random_in_unit_sphere();
        Some((Ray(rec.p, target - rec.p), self.0))
    }
}

pub struct Metal {
    albedo: Vec3,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f32) -> Self {
        Metal {
            albedo,
            fuzz: if fuzz < 0.0 {
                0.0
            } else if fuzz > 1.0 {
                1.0
            } else {
                fuzz
            },
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Vec3)> {
        let reflected = r_in.direction().into_unit().reflect_about(rec.normal);
        let scattered = Ray(rec.p, reflected + self.fuzz * Vec3::random_in_unit_sphere());
        if scattered.direction().dot(rec.normal) > 0.0 {
            return Some((scattered, self.albedo));
        }
        None
    }
}

pub struct Dielectric {
    pub ref_idx: f32,
}

fn refract(v: &Vec3, n: &Vec3, ni_over_nt: f32) -> Option<Vec3> {
    let uv = v.into_unit();
    let dt = uv.dot(*n);
    let disc = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if disc > 0.0 {
        return Some(ni_over_nt * (uv - dt * *n) - disc.sqrt() * *n);
    }
    None
}

fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * ((1.0 - cosine).powf(5.0))
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Vec3)> {
        let attenuation = Vec3::from(1.0);
        let disc = r_in.direction().dot(rec.normal) > 0.0;
        let outward_normal = if disc {
            -rec.normal
        } else {
            rec.normal
        };
        let ni_over_nt = if disc {
            self.ref_idx
        } else {
            1.0 / self.ref_idx
        };
        let cosine = if disc {
            self.ref_idx * r_in.direction().dot(rec.normal) / r_in.direction().length()
        } else {
            -r_in.direction().dot(rec.normal) / r_in.direction().length()
        };
        let mut rng = rand::thread_rng();
        let get_reflection_result = || (Ray(rec.p, r_in.direction().reflect_about(rec.normal)), attenuation);
        match refract(&r_in.direction(), &outward_normal, ni_over_nt) {
            Some(refracted) => {
                if rng.gen::<f32>() < schlick(cosine, self.ref_idx) {
                    return Some(get_reflection_result());
                } else {
                    return Some((Ray(rec.p, refracted), attenuation));
                }
            },
            None => {
                return Some(get_reflection_result());
            }
        }
    }
}
