use image;
use wknder::vec3::{Vec3, Axis::*};
use wknder::sphere::Sphere;
use wknder::ray::Ray;
use wknder::hittable::*;
use wknder::camera::Camera;
use wknder::material::{Lambertian, Metal, Dielectric};
use rand::Rng;
use std::rc::Rc;
use std::vec;

const WIDTH: u32 = 1920;
const HEIGHT: u32 = 1080;
const AA_SAMPLES: u32 = 100;
const MAX_BOUNCES: u32 = 50;

fn main() {
    let mut buf = vec![];

    //let world = basic_scene();
    let world = random_scene();

    let lookfrom = Vec3(8.0, 2.0, 2.5);
    let lookat = Vec3(0.0, 0.0, -1.0);
    let dist_to_focus = (lookfrom - lookat).length();
    let aperture = 0.1;
    let cam = Camera::new(lookfrom, lookat, Vec3(0.0, 1.0, 0.0), 35.0, WIDTH as f32 / HEIGHT as f32, aperture, dist_to_focus);

    let mut rng = rand::thread_rng();

    for j in (0..HEIGHT).rev() {
        for i in 0..WIDTH {
            let mut col = Vec3::from(0.0);
            for _ in 0..AA_SAMPLES {
                let u = (i as f32 + rng.gen::<f32>()) / WIDTH as f32;
                let v = (j as f32 + rng.gen::<f32>()) / HEIGHT as f32;
                let r = cam.get_ray(u, v);
                col += color(&r, &world, 0);
            }
            col /= AA_SAMPLES as f32;
            col.each(|x| buf.push(float_to_byte(x.sqrt())));
        }
        println!("Traced {} of {} vertical lines ({}%)", (HEIGHT - j), HEIGHT, ((HEIGHT - j) as f32 / HEIGHT as f32) * 100.0);
    }

    image::save_buffer("output.png", &buf, WIDTH, HEIGHT, image::ColorType::RGB(8)).unwrap();
}

fn color(r: &Ray, world: &impl Hittable, depth: u32) -> Vec3 {
    let mut rec = HitRecord::empty();
    if world.hit(r, 0.001, std::f32::MAX, &mut rec) {
        if depth >= MAX_BOUNCES {
            return Vec3::from(0.0);
        }
        
        let material = match &rec.material {
            Some(m) => m,
            None => return Vec3::from(0.0),
        };

        if let Some((scattered, attenuation)) = material.scatter(r, &rec) {
            return attenuation * color(&scattered, world, depth + 1);
        }
    }
    
    let unit_direction = r.direction().into_unit();
    let t = 0.5 * (unit_direction[Y] + 1.0);
    (1.0 - t) * Vec3::from(1.0) + t * Vec3(0.5, 0.7, 1.0)
}

#[inline]
fn float_to_byte(f: f32) -> u8 {
    (f * 255.99) as u8
}

fn basic_scene() -> HittableList {
    HittableList::new(vec![
        Box::new(Sphere::new(
            Vec3(0.0, 0.0, -1.0),
            0.5,
            Rc::new(Lambertian(Vec3(0.8, 0.3, 0.3)))
        )),
        Box::new(Sphere::new(
            Vec3(0.0, -100.5, -1.0),
            100.0,
            Rc::new(Lambertian(Vec3(0.8, 0.8, 0.0)))
        )),
        Box::new(Sphere::new(
            Vec3(1.0, 0.0, -1.0),
            0.5,
            Rc::new(Metal::new(Vec3(0.8, 0.6, 0.2), 1.0))
        )),
        Box::new(Sphere::new(
            Vec3(-1.0, 0.0, -1.0),
            0.5,
            Rc::new(Dielectric { ref_idx: 1.5 })
        )),
        Box::new(Sphere::new(
            Vec3(-1.0, 0.0, -1.0),
            -0.45,
            Rc::new(Dielectric { ref_idx: 1.5 })
        )),
    ])
}

fn random_scene() -> HittableList {
    let mut rng = rand::thread_rng();
    let mut list: Vec<Box<dyn Hittable>> = vec![
        Box::new(
            Sphere::new(
                Vec3(0.0, -1000.0, 0.0),
                1000.0,
                Rc::new(Lambertian(Vec3::from(0.5)))
            )
        )
    ];
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f32 = rng.gen();
            let center = Vec3(a as f32 + 0.9 * rng.gen::<f32>(), 0.2, b as f32 + 0.9 * rng.gen::<f32>());
            if (center - Vec3(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    list.push(Box::new(Sphere::new(
                        center, 0.2,
                        Rc::new(Lambertian(Vec3(
                            rng.gen::<f32>() * rng.gen::<f32>(),
                            rng.gen::<f32>() * rng.gen::<f32>(),
                            rng.gen::<f32>() * rng.gen::<f32>(),
                        )))
                    )));
                } else if choose_mat < 0.95 {
                    list.push(Box::new(Sphere::new(
                        center, 0.2,
                        Rc::new(Metal::new(Vec3(
                            0.5 * (1.0 + rng.gen::<f32>()),
                            0.5 * (1.0 + rng.gen::<f32>()),
                            0.5 * (1.0 + rng.gen::<f32>()),
                        ), 0.5 * rng.gen::<f32>()))
                    )));
                } else {
                    list.push(Box::new(Sphere::new(
                        center, 0.2,
                        Rc::new(Dielectric { ref_idx: 1.5 })
                    )));
                }
            }
        }
    }

    list.push(Box::new(Sphere::new(
        Vec3(0.0, 1.0, 0.0),
        1.0,
        Rc::new(Dielectric { ref_idx: 1.5 })
    )));
    list.push(Box::new(Sphere::new(
        Vec3(-4.0, 1.0, 0.0),
        1.0,
        Rc::new(Lambertian(Vec3(0.4, 0.2, 0.1)))
    )));
    list.push(Box::new(Sphere::new(
        Vec3(4.0, 1.0, 0.0),
        1.0,
        Rc::new(Metal::new(Vec3(0.7, 0.6, 0.5), 0.0))
    )));

    HittableList::new(list)
}
