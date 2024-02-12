use std::fs::File;
use std::io::{stdout, BufWriter, Write};

use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::color::write_color;
use crate::surface::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::world::World;
use crate::materials::{Materials, Scatter};

pub struct Camera {
    look_at: Vec3,                                                              // Point the camera is looking at
    look_from: Vec3,                                                            // Point where the camera is
    up_dir: Vec3,                                                               // Camera's relative up direction
    max_d: u8,                                                                  // Max depth (n° of jumps)
    vfov: f64,                                                                  // Vertical FOV
    ar: f64,                                                                    // Aspect ratio
    f: f64,                                                                     // Focal length
    w: u16,                                                                     // Screen image_width
    h: u16,                                                                     // Screen height
    vh: f64,                                                                    // Viewport image_width
    vw: f64,                                                                    // Viewport height
    vu: Vec3,                                                                   // V_u
    vv: Vec3,                                                                   // V_v
    du: Vec3,                                                                   // delta_u between pixels
    dv: Vec3,                                                                   // delta_v between pixels
    center: Vec3,                                                               // Camera center
    v_corner: Vec3,                                                             // Viewport upper-left corner location
    aa_sqrt: u8,                                                                // sqrt(Samples per pixel) for antialiasing
    aa: u8,                                                                     // Samples per pixel for antialiasing
    dus: Vec3,                                                                  // delta_u between samples
    dvs: Vec3,                                                                  // delta_v between samples
    s_corner: Vec3,                                                             // Position of the first top-left sample
}

impl Camera {
    pub fn new(
        look_at: Vec3,
        look_from: Vec3,
        up_dir: Vec3,
        aspect_ratio: f64, 
        image_width: u16, 
        max_depth: u8, 
        vertical_fov: f64,
        aa_factor: u8
    ) -> Camera {
        let k: Vec3 = (look_from - look_at).unit();                             // Camera coordinate frame unit basis vector w
        let i: Vec3 = (up_dir.x(k)).unit();                                     // Camera coordinate frame unit basis vector u
        let j: Vec3 = k.x(i);                                                   // Camera coordinate frame unit basis vector v

        let focal_length: f64 = (look_from - look_at).len();
        let h: u16 = (image_width as f64/aspect_ratio) as u16;
        let theta: f64 = vertical_fov.to_radians();
        let vfov_h_ratio: f64 = (theta/2.).tan();
        let vh: f64 = 2.*vfov_h_ratio*focal_length;
        let vw: f64 = vh*(image_width as f64/h as f64);        
        let vu: Vec3 = vw*i;
        let vv: Vec3 = -vh*j;
        let du: Vec3 = vu/(image_width as f64);
        let dv: Vec3 = vv/(h as f64);
        let center: Vec3 = look_from;
        let v_corner: Vec3 = center - (focal_length*k) - vu/2. - vv/2.;
        let aa_sqrt: u8 = (aa_factor as f64).sqrt() as u8;
        let aa: u8 = aa_sqrt*aa_sqrt;
        let dus: Vec3 = du/(aa_sqrt + 1) as f64;
        let dvs: Vec3 = dv/(aa_sqrt + 1) as f64;
        let s_corner: Vec3 = v_corner + dus + dvs;
        
        Camera {
            look_at: look_at,
            look_from: look_from,
            up_dir: up_dir,
            max_d: max_depth,
            vfov: theta,
            ar: aspect_ratio,
            f: focal_length,
            w: image_width,
            h: h,
            vh: vh,
            vw: vw,
            center: center,
            vu: vu,
            vv: vv,
            du: du,
            dv: dv,
            v_corner: v_corner,
            aa_sqrt: aa_sqrt,
            aa: aa,
            dus: dus,
            dvs: dvs,
            s_corner: s_corner
        }
    }

    pub fn render(&self, world: World) {
        let pixels_total: u32 = (self.w as u32*self.h as u32);
        let mut pixels_done: u32 = 0;
        let file: File = File::create("./output.ppm").expect("Creation failed");
        let mut writer = BufWriter::new(&file);
        writeln!(&mut writer, "P3\n{} {}\n255", self.w, self.h);
        for j in 0..self.h {
            for i in 0..self.w {
                write_color(&mut writer, self.get_px_color(&world, i, j));
                
                pixels_done += 1;
                print!("\r{:.2}%", 100.*pixels_done as f32/pixels_total as f32);
                stdout().flush();
            }
        }
    }

    fn get_px_color(&self, world: &World, i: u16, j: u16) -> Vec3 {
        let mut px_color: Vec3 = Vec3(0., 0., 0.);
        let start: Vec3 = self.s_corner + (i as f64)*self.du + (j as f64)*self.dv;
        for pi in 0..self.aa_sqrt {
            for pj in 0..self.aa_sqrt {
                let pos: Vec3 = start + (pi as f64)*self.dus + (pj as f64)*self.dvs;
                let r: Ray = Ray {ori: self.center, dir: pos - self.center};
                px_color += Self::ray_color(r, self.max_d, &world)/self.aa as f64;
            }
        }
        px_color
    }

    fn ray_color(r: Ray, depth: u8, world: &World) -> Vec3 {
        if (depth <= 0) {
            return Vec3(0. ,0., 0.);
        }

        let mut rec: HitRecord = HitRecord::new_empty();
        if (world.hit(r, Interval{min: 0.000001, max: f64::INFINITY}, &mut rec)) {
            let (r_out, color): (Ray, Vec3) = rec.mat.scatter(r, &rec); 
            if (!color.near_zero()) {
                color.coord_mul(Self::ray_color(r_out, depth-1, world))
            } else {
                Vec3(0., 0., 0.)
            }
        } else {
            let unit_dir: Vec3 = r.dir.unit();
            let a: f64 = 0.5*(unit_dir.1 + 1.0);
            (1.0 - a)*Vec3(1.0, 1.0, 1.0) + a*Vec3(0.5, 0.7, 1.0)
        }
    }
}