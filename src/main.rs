#![allow(unused)]
#![allow(unused_variables)]
#![allow(dead_code)]

pub mod vec3;
pub mod ray;
pub mod color;
pub mod surface;
pub mod sphere;
pub mod interval;
pub mod world;
pub mod camera;
pub mod random;

use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::color::write_color;
use crate::surface::{HitRecord, Hittable};
use crate::sphere::Sphere;
use crate::interval::Interval;
use crate::world::{World, Surfaces};
use crate::camera::Camera;

use std::f64::consts::PI;

fn main() {
    let mut world: World = World {surfaces: vec![
        Surfaces::Sphere(Sphere {center: Vec3(0.18, 0., -0.325), radius: 0.076}),
        Surfaces::Sphere(Sphere {center: Vec3(0., 0., -0.8), radius: 0.5}),
        Surfaces::Sphere(Sphere {center: Vec3(0., -100.5, -1.), radius: 100.}),
        Surfaces::Sphere(Sphere {center: Vec3(0.13, 0., -0.475), radius: 0.2})
    ]};

    let camera: Camera = Camera::new(16.0/9.0, 600, 3, 1.0, 16);
    camera.render(world);
}