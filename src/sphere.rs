use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::surface::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::materials::Materials;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub mat: Materials
}

impl Hittable for Sphere {
    fn hit(&self, r: Ray, t_range: Interval, rec: &mut HitRecord) -> bool {
        let oc: Vec3 = r.ori - self.center;
        let a: f64 = r.dir.l2norm();
        let half_b: f64 = oc*r.dir;
        let c: f64 = oc.l2norm() - self.radius*self.radius;
        let discriminant: f64 = half_b*half_b - a*c;
        if (discriminant < 0.) { 
            return false;
        }

        let sqrt_d: f64 = discriminant.sqrt();
        let mut root: f64 = (-half_b - sqrt_d)/a;                                   // Value of t for the nearest sphere hit 
        if (!t_range.surrounds(root)) {
            root = (-half_b + sqrt_d)/a;
            if (!t_range.surrounds(root)) {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal: Vec3 = (rec.p - self.center)/self.radius;
        rec.set_face_normal(r, outward_normal);
        rec.mat = self.mat;
        true
    }
}