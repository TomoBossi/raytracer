use crate::vec3::Vec3;
use crate::ray::Ray;

pub struct HitRecord {
    pub p: Vec3,
    pub n: Vec3,
    pub t: f32
}

impl HitRecord {
    pub fn new_empty() -> HitRecord {
        HitRecord {
            p: Vec3(0., 0., 0.),
            n: Vec3(0., 0., 0.),
            t: 0.
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: Ray, r_tmin: f32, r_tmax: f32, rec: &mut HitRecord) -> bool;
}