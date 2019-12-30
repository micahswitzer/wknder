use crate::vec3::{Vec3, Axis::*};
use crate::ray::Ray;
use std::f32::consts;

#[derive(Copy, Clone, Debug)]
pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f32, 
}

impl Camera {
    pub fn new(lookfrom: Vec3, lookat: Vec3, vup: Vec3, vfov: f32, aspect: f32, aperture: f32, focus_dist: f32) -> Self {
        let theta = vfov * consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        let w = (lookfrom - lookat).into_unit();
        let u = vup.cross(&w).into_unit();
        let v = w.cross(&u);
        Camera {
            origin: lookfrom,
            lower_left_corner: lookfrom - half_width * focus_dist * u - half_height * focus_dist * v - focus_dist * w,
            horizontal: 2.0 * half_width * focus_dist * u,
            vertical: 2.0 * half_height * focus_dist * v,
            u, v, w,
            lens_radius: aperture / 2.0,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let rd = self.lens_radius * Vec3::random_in_unit_disk();
        let offset = rd[X] * self.u + rd[Y] * self.v;
        Ray(self.origin + offset, self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin - offset)
    }
}
