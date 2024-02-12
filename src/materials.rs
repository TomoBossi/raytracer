use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::surface::HitRecord;
use crate::random::random;

#[derive(Clone, Copy)]
pub enum Materials { // Innecesario, no necesito armar vector de materiales... considerar reemplazar por trait donde usado
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric)
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

#[derive(Clone, Copy)]
pub struct Dielectric {
    pub color: Vec3,
    pub refraction_idx: f64
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

impl Scatter for Dielectric {
    fn scatter(&self, r_in: Ray, rec: &HitRecord) -> (Ray, Vec3) {              // Scattered ray, color, reflectance (attenuation)
        let idx_ratio: f64 = if (rec.front) {1./self.refraction_idx} else {self.refraction_idx};
        let unit_dir: Vec3 = r_in.dir.unit();
        let cos_theta: f64 = f64::min(-unit_dir*rec.n, 1.);
        let sin_theta: f64 = (1. - cos_theta*cos_theta).sqrt();
        let cannot_refract: bool = idx_ratio*sin_theta > 1.;
        if (cannot_refract || Self::reflectance(cos_theta, idx_ratio) > random()) {
            (Ray {ori: rec.p, dir: unit_dir.reflect(rec.n)}, Vec3(1., 1., 1.))
        } else {
            (Ray {ori: rec.p, dir: unit_dir.refract(rec.n, idx_ratio)}, self.color)
        }
    }
}

impl Scatter for Materials {
    fn scatter(&self, r_in: Ray, rec: &HitRecord) -> (Ray, Vec3) {
        match self {
            Materials::Lambertian(l) => l.scatter(r_in, rec),
            Materials::Metal(m) => m.scatter(r_in, rec),
            Materials::Dielectric(d) => d.scatter(r_in, rec)
        }
    }
}

impl Dielectric {
    fn reflectance(cos: f64, reflectance_idx: f64) -> f64 {
        let r0_sqrt: f64 = (1. - reflectance_idx)/(1. + reflectance_idx);
        let r0: f64 = r0_sqrt*r0_sqrt;
        r0 + (1. - r0)*(1. - cos).powi(5)
    }
}