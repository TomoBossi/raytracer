#![allow(unused)]
#![allow(unused_variables)]
#![allow(dead_code)]

use std::env;

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
use crate::surface::{HitRecord, Hittable};
use crate::sphere::Sphere;
use crate::interval::Interval;
use crate::world::{World, Surfaces};
use crate::camera::Camera;
use crate::random::{random, random_in};
use crate::materials::{Materials, Lambertian, Metal, Dielectric};

fn main() { // cargo run --release -- out/out.png
    let args: Vec<String> = env::args().collect();
    let output_file: String = args[1].clone();

    let mut world: World = World {surfaces: vec![]};
    let ground_material: Materials = Materials::Lambertian(Lambertian {color: Vec3(0.5, 0.5, 0.5)});
    let material1: Materials = Materials::Dielectric(Dielectric {color: Vec3(1., 1., 1.), refraction_idx: 1.5});
    let material2: Materials = Materials::Lambertian(Lambertian {color: Vec3(0.4, 0.2, 0.1)});
    let material3: Materials = Materials::Metal(Metal {color: Vec3(0.7, 0.6, 0.5), fuzz: 0.});

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = random();
            let center: Vec3 = Vec3(a as f64 + 0.9*random(), 0.2, b as f64 + 0.9*random());
            if ((center - Vec3(4., 0.2, 0.)).len() > 0.9) {
                let mut sphere_material: Materials;
                if (choose_mat < 0.8) {
                    // diffuse
                    let albedo: Vec3 = Vec3::random().coord_mul(Vec3::random());
                    sphere_material = Materials::Lambertian(Lambertian {color: albedo});
                    world.surfaces.push(Surfaces::Sphere(Sphere {center: center, radius: 0.2, mat: sphere_material}));
                } else if (choose_mat < 0.95) {
                    // metal
                    let albedo: Vec3 = Vec3::random_in(0.5, 1.);
                    let fuzz: f64 = random_in(0., 0.5);
                    sphere_material = Materials::Metal(Metal {color: albedo, fuzz: fuzz});
                    world.surfaces.push(Surfaces::Sphere(Sphere {center: center, radius: 0.2, mat: sphere_material}));
                } else {
                    // glass
                    sphere_material = Materials::Dielectric(Dielectric {color: Vec3(1., 1., 1.), refraction_idx: 1.5});
                    world.surfaces.push(Surfaces::Sphere(Sphere {center: center, radius: 0.2, mat: sphere_material}));
                }
            }
        }
    }

    world.surfaces.push(Surfaces::Sphere(Sphere {center: Vec3(0., -1000., 0.), radius: 1000., mat: ground_material}));
    world.surfaces.push(Surfaces::Sphere(Sphere {center: Vec3(0., 1., 0.), radius: 1., mat: material1}));
    world.surfaces.push(Surfaces::Sphere(Sphere {center: Vec3(-4., 1., 0.), radius: 1., mat: material2}));
    world.surfaces.push(Surfaces::Sphere(Sphere {center: Vec3(4., 1., 0.), radius: 1., mat: material3}));

    let camera: Camera = Camera::new(
        output_file,
        Vec3(0., 0., 0.),
        Vec3(13., 2., 3.), 
        Vec3(0., 1., 0.),
        0.65,
        10.,
        16.0/9.0,
        2000,
        50,
        20.,
        64
    );

    camera.render(world);
}