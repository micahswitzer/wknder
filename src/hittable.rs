use crate::{vec3::Vec3, ray::Ray, material::Material};
use std::rc::Rc;

pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: Option<Rc<dyn Material>>,
}

impl HitRecord {
    pub fn empty() -> HitRecord {
        HitRecord {
            t: 0.0,
            p: Vec3::from(0.0),
            normal: Vec3::from(0.0),
            material: Option::<Rc<dyn Material>>::None,
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool;
}

pub struct HittableList {
    list: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new(list: Vec<Box<dyn Hittable>>) -> HittableList {
        HittableList { list }
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::empty();
        let mut closest_so_far = t_max;
        let mut hit_anything = false;
        for hittable in self.list.iter() {
            if hittable.hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                rec.t = temp_rec.t;
                rec.p = temp_rec.p;
                rec.normal = temp_rec.normal;
                rec.material = match &temp_rec.material {
                    Some(m) => Some(Rc::clone(&m)),
                    None => None
                };
            }
        }
        hit_anything
    }
}
