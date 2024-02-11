use crate::vec3::Vec3;
use crate::ray::Ray;

pub struct HitRecord {
    pub p: Vec3,
    pub n: Vec3,
    pub t: f32,
    pub front: bool
}

impl HitRecord {
    pub fn new_empty() -> HitRecord {
        HitRecord {
            p: Vec3(0., 0., 0.),
            n: Vec3(0., 0., 0.),
            t: 0.,
            front: true
        }
    }

    pub fn set_face_normal(&mut self, r: Ray, outward_normal: Vec3) {
        self.front = r.dir*outward_normal < 0.;
        self.n = (if (self.front) {1.} else {-1.})*outward_normal;
    }
}

pub trait Hittable {
    fn hit(&self, r: Ray, r_tmin: f32, r_tmax: f32, rec: &mut HitRecord) -> bool;
}

impl std::clone::Clone for HitRecord { // rec.clone();
    fn clone(&self) -> Self {
        HitRecord {
            p: self.p.clone(),
            n: self.n.clone(),
            t: self.t.clone(),
            front: self.front.clone()
        }
    }
}

impl std::marker::Copy for HitRecord {}