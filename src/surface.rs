use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::interval::Interval;
use crate::materials::{Materials, Lambertian};

pub trait Hittable {
    fn hit(&self, r: Ray, t_range: Interval, rec: &mut HitRecord) -> bool;
}

#[derive(Clone, Copy)]
pub struct HitRecord {
    pub p: Vec3,
    pub n: Vec3,
    pub t: f64,
    pub front: bool,
    pub mat: Materials
}

impl HitRecord {
    pub fn new_empty() -> HitRecord {
        HitRecord {
            p: Vec3(0., 0., 0.),
            n: Vec3(0., 0., 0.),
            t: f64::INFINITY,
            front: true,
            mat: Materials::Lambertian(Lambertian {
                color: Vec3(0., 0., 0.)
            })
        }
    }

    pub fn set_face_normal(&mut self, r: Ray, outward_normal: Vec3) {
        self.front = r.dir*outward_normal < 0.;
        self.n = (if (self.front) {1.} else {-1.})*outward_normal;
    }
}