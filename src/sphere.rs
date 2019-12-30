use crate::{vec3::Vec3, ray::Ray, hittable::*, material::Material};
use std::sync::Arc;

pub struct Sphere {
    center: Vec3,
    radius: f32,
    material: Arc<dyn Material + Sync + Send>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Arc<dyn Material + Sync + Send>) -> Self {
        Sphere {
            center,
            radius,
            material
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = r.direction().dot(r.direction());
        let b = oc.dot(r.direction());
        let c = oc.dot(oc) - self.radius * self.radius;
        let disc = b * b - a * c;
        if disc > 0.0 {
            let temp = (-b - disc.sqrt()) / a;
            if temp < t_max && temp > t_min {
                let p = r.point_at_parameter(temp);
                return Some(HitRecord {
                    t: temp,
                    p,
                    normal: (p - self.center) / self.radius,
                    material: Some(Arc::clone(&self.material)),
                });
            }
            let temp = (-b + disc.sqrt()) / a;
            if temp < t_max && temp > t_min {
                let p = r.point_at_parameter(temp);
                return Some(HitRecord {
                    t: temp,
                    p, 
                    normal: (p - self.center) / self.radius,
                    material: Some(Arc::clone(&self.material)),
                });
            }
        }
        None
    }
}
