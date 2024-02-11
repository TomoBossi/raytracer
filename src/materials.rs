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
    pub color: Vec3
}

impl Scatter for Lambertian {
    fn scatter(&self, r_in: Ray, rec: &HitRecord) -> (Ray, Vec3) {
        let mut scatter_direction: Vec3 = rec.n + Vec3::random_unit();
        if (scatter_direction.near_zero()) {
            scatter_direction = rec.n;
        }
        (Ray {ori: rec.p, dir: scatter_direction}, self.color)
    }
}

impl Scatter for Metal {
    fn scatter(&self, r_in: Ray, rec: &HitRecord) -> (Ray, Vec3) {              // Scattered ray, color, reflectance (attenuation)
        let mut reflected_direction: Vec3 = r_in.dir.unit().reflect(rec.n);
        (Ray {ori: rec.p, dir: reflected_direction}, self.color)
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