use crate::{vec3::Vec3, ray::Ray, hittable::*, material::Material};
use std::rc::Rc;

pub struct Sphere {
    center: Vec3,
    radius: f32,
    material: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Rc<dyn Material>) -> Self {
        Sphere {
            center,
            radius,
            material
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let oc = r.origin() - self.center;
        let a = r.direction().dot(r.direction());
        let b = oc.dot(r.direction());
        let c = oc.dot(oc) - self.radius * self.radius;
        let disc = b * b - a * c;
        if disc > 0.0 {
            let temp = (-b - disc.sqrt()) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.point_at_parameter(temp);
                rec.normal = (rec.p - self.center) / self.radius;
                rec.material = Some(Rc::clone(&self.material));
                return true;
            }
            let temp = (-b + disc.sqrt()) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.point_at_parameter(temp);
                rec.normal = (rec.p - self.center) / self.radius;
                rec.material = Some(Rc::clone(&self.material));
                return true;
            }
        }
        false
    }
}
