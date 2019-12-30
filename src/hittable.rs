use crate::{vec3::Vec3, ray::Ray, material::Material};
use std::sync::Arc;

pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: Option<Arc<dyn Material + Sync + Send>>,
}

impl HitRecord {
    pub fn empty() -> HitRecord {
        HitRecord {
            t: 0.0,
            p: Vec3::from(0.0),
            normal: Vec3::from(0.0),
            material: Option::<Arc<dyn Material + Sync + Send>>::None,
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

pub struct HittableList {
    list: Vec<Box<dyn Hittable + Sync + Send>>,
}

impl HittableList {
    pub fn new(list: Vec<Box<dyn Hittable + Sync + Send>>) -> HittableList {
        HittableList { list }
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut rec: Option<HitRecord> = None;
        for hittable in self.list.iter() {
            if let Some(temp_rec) = hittable.hit(r, t_min, closest_so_far) {
                closest_so_far = temp_rec.t;
                rec = Some(temp_rec);
            }
        }
        rec
    }
}
