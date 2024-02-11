use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::surface::{HitRecord, Hittable};
use crate::sphere::Sphere;
use crate::interval::Interval;

pub enum Surfaces {
    Sphere(Sphere)
}

impl Hittable for Surfaces {
    fn hit(&self, r: Ray, t_range: Interval, rec: &mut HitRecord) -> bool {
        match self {
            Surfaces::Sphere(sphere) => sphere.hit(r, t_range, rec)
        }
    }
}

pub struct World {
    pub surfaces: Vec<Surfaces>
}

impl Hittable for World {
    fn hit(&self, r: Ray, t_range: Interval, rec: &mut HitRecord) -> bool {
        let mut temp_rec: HitRecord = HitRecord::new_empty();
        let mut world_hit: bool = false;
        let mut nearest: f32 = t_range.max;
        for surface in self.surfaces.iter() {
            if (surface.hit(r, Interval{min: 0., max: nearest}, &mut temp_rec)) {
                nearest = temp_rec.t;
                world_hit = true;
                *rec = temp_rec;
            }
        }
        world_hit
    }
}