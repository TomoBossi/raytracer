#![allow(unused)]
#![allow(unused_variables)]
#![allow(dead_code)]

pub mod vec3;
pub mod ray;
pub mod color;

use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::color::write_color;

fn main() {
    let aspect_ratio: f32 = 16.0/9.0;                                           // Aspect ratio
    let w: u16 = 400;                                                           // Screen width
    let h: u16 = (w as f32/aspect_ratio) as u16;                                // Screen height
    let vh: f32 = 2.0;                                                          // Viewport width
    let vw: f32 = vh*(w as f32/h as f32);                                       // Viewport height
    let focal_len: f32 = 1.0;                                                   // Focal length
    let cam_center: Vec3 = Vec3(0., 0., 0.);                                    // Camera center
    let vu: Vec3 = Vec3(vw as f32, 0., 0.);                                     // V_u
    let vv: Vec3 = Vec3(0., -vh as f32, 0.);                                    // V_v
    let du: Vec3 = vu/(w as f32);                                               // delta_u
    let dv: Vec3 = vv/(h as f32);                                               // delta_v
    let v_corner: Vec3 = cam_center - Vec3(0., 0., focal_len) - vu/2. - vv/2.;  // Viewport upper-left corner location
    let px00_loc: Vec3 = v_corner + (du + dv)/2.;                               // Pixel (0, 0) location

    let mut px_center: Vec3;
    let mut px_color: Vec3;
    let mut ray_dir: Vec3;
    let mut r: Ray;
    println!("P3\n{} {}\n255", w, h);
    for j in 0..h {
        for i in 0..w {
            px_center = px00_loc + (i as f32)*du + (j as f32)*dv;
            ray_dir = px_center - cam_center;
            r = Ray{ori: cam_center, dir: ray_dir};
            px_color = ray_color(r);
            write_color(px_color);
        }
    }
}


fn ray_color(r: Ray) -> Vec3 {
    if (sphere(Vec3(0., 0., -1.), 0.5, r)) {
        return Vec3(1., 0., 0.);
    }

    let unit_dir: Vec3 = r.dir.unit();
    let a: f32 = 0.5*(unit_dir.1 + 1.0);
    (1.0 - a)*Vec3(1.0, 1.0, 1.0) + a*Vec3(0.5, 0.7, 1.0)
}


fn sphere(s_center: Vec3, radius: f32, r: Ray) -> bool {
    let oc: Vec3 = r.ori - s_center;
    let a: f32 = r.dir*r.dir;
    let b: f32 = 2.0*oc*r.dir;
    let c: f32 = oc*oc - radius*radius;
    let discriminant: f32 = b*b - 4.*a*c;
    discriminant >= 0.
}


fn output_an_image(w: u16, h: u16) {
    let mut rgb: Vec3;
    println!("P3\n{} {}\n255", w, h);
    for j in 0..h {
        for i in 0..w {
            rgb = Vec3(i as f32/(w-1) as f32, j as f32/(h-1) as f32, 0.5);
            write_color(rgb);
        }
    }
}