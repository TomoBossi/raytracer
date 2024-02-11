use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::surface::{HitRecord, Hittable};

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32
}

impl Hittable for Sphere {
    fn hit(&self, r: Ray, r_tmin: f32, r_tmax: f32, rec: &mut HitRecord) -> bool {
        let oc: Vec3 = r.ori - self.center;
        let a: f32 = r.dir.l2norm();
        let half_b: f32 = oc*r.dir;
        let c: f32 = oc.l2norm() - self.radius*self.radius;
        let discriminant: f32 = half_b*half_b - a*c;
        if (discriminant < 0.) { 
            return false;
        }

        let sqrt_d: f32 = discriminant.sqrt();
        let root: f32 = (-half_b - sqrt_d)/a;                                   // Value of t for the nearest sphere hit 
        if (root <= r_tmin || r_tmax <= root) {
            let root = (-half_b + sqrt_d)/a;
            if (root <= r_tmin || r_tmax <= root) {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal: Vec3 = (rec.p - self.center)/self.radius;
        rec.set_face_normal(r, outward_normal);
        true
    }
}