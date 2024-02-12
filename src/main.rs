#![allow(unused)]
#![allow(unused_variables)]
#![allow(dead_code)]

use std::f64::consts::PI;

pub mod vec3;
pub mod ray;
pub mod color;
pub mod surface;
pub mod sphere;
pub mod interval;
pub mod world;
pub mod camera;
pub mod random;
pub mod materials;

use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::color::write_color;
use crate::surface::{HitRecord, Hittable};
use crate::sphere::Sphere;
use crate::interval::Interval;
use crate::world::{World, Surfaces};
use crate::camera::Camera;
use crate::materials::{Materials, Lambertian, Metal, Dielectric};

fn main() {
    let grass: Materials = Materials::Lambertian(Lambertian {color: Vec3(0.2, 0.8, 0.3)});
    let blue: Materials = Materials::Lambertian(Lambertian {color: Vec3(0.2, 0.3, 1.0)});
    let red: Materials = Materials::Lambertian(Lambertian {color: Vec3(0.9, 0.2, 0.3)});
    let metal: Materials = Materials::Metal(Metal {color: Vec3(0.8, 0.8, 0.8), fuzz: 0.01});
    let hue_metal: Materials = Materials::Metal(Metal {color: Vec3(0.8, 0.6, 0.2), fuzz: 0.02});
    let glass: Materials = Materials::Dielectric(Dielectric {color: Vec3(1., 1., 1.), refraction_idx: 1.5});

    let mut world: World = World {surfaces: vec![
        Surfaces::Sphere(Sphere {center: Vec3(0., -100.5, -1.), radius: 100., mat: grass}),
        Surfaces::Sphere(Sphere {center: Vec3(0., 0., -1.), radius: 0.5, mat: glass}),
        Surfaces::Sphere(Sphere {center: Vec3(0., 0., -1.), radius: -0.25, mat: glass}),
        Surfaces::Sphere(Sphere {center: Vec3(0.6, -0.4, -0.7), radius: 0.125, mat: red}),
        Surfaces::Sphere(Sphere {center: Vec3(-0.6, -0.5, -0.7), radius: 0.3, mat: metal}),
        Surfaces::Sphere(Sphere {center: Vec3(1., 0.5, -1.5), radius: 0.3, mat: hue_metal}),
        Surfaces::Sphere(Sphere {center: Vec3(-0.1, -0.4, -0.56), radius: 0.05, mat: metal}),
    ]};

    let camera: Camera = Camera::new(
        Vec3(0., 0., -1.),
        Vec3(-2., 2., 1.), 
        Vec3(0., 1., 0.),
        9.0/16.0,
        100,
        10,
        40.,
        64
    );

    camera.render(world);
}