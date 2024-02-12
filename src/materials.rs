use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::surface::HitRecord;

#[derive(Clone, Copy)]
pub enum Materials {
    Lambertian(Lambertian),
    Metal(Metal)
}

pub trait Scatter {
    fn scatter(&self, r_in: Ray, rec: &HitRecord) -> (Ray, Vec3); // Scattered ray, color (attenuations)
}

#[derive(Clone, Copy)]
pub struct Lambertian {
    pub color: Vec3
}

#[derive(Clone, Copy)]
pub struct Metal {
    pub color: Vec3,
    pub fuzz: f64
}

impl Scatter for Lambertian {
    fn scatter(&self, r_in: Ray, rec: &HitRecord) -> (Ray, Vec3) {
        let mut scattered_dir: Vec3 = rec.n + Vec3::random_unit();
        if (scattered_dir.near_zero()) {
            scattered_dir = rec.n;
        }
        (Ray {ori: rec.p, dir: scattered_dir}, self.color)
    }
}

impl Scatter for Metal {
    fn scatter(&self, r_in: Ray, rec: &HitRecord) -> (Ray, Vec3) {              // Scattered ray, color, reflectance (attenuation)
        let reflected_dir: Vec3 = r_in.dir.unit().reflect(rec.n);
        let scattered_dir: Vec3 = reflected_dir + self.fuzz*Vec3::random_unit();
        (Ray {ori: rec.p, dir: scattered_dir}, self.color)
    }
}

impl Scatter for Materials {
    fn scatter(&self, r_in: Ray, rec: &HitRecord) -> (Ray, Vec3) {
        match self {
            Materials::Lambertian(l) => l.scatter(r_in, rec),
            Materials::Metal(m) => m.scatter(r_in, rec),
        }
    }
}