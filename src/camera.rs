use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::color::write_color;
use crate::surface::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::world::World;
use crate::materials::{Materials, Scatter};

pub struct Camera {
    max_d: u8,                                                                  // Max depth (n° of jumps)
    ar: f64,                                                                    // Aspect ratio
    f: f64,                                                                     // Focal length
    w: u16,                                                                     // Screen viewport_width
    h: u16,                                                                     // Screen height
    vh: f64,                                                                    // Viewport viewport_width
    vw: f64,                                                                    // Viewport height
    vu: Vec3,                                                                   // V_u
    vv: Vec3,                                                                   // V_v
    du: Vec3,                                                                   // delta_u between pixels
    dv: Vec3,                                                                   // delta_v between pixels
    c_center: Vec3,                                                             // Camera center
    v_corner: Vec3,                                                             // Viewport upper-left corner location
    aa_sqrt: u8,                                                                // sqrt(Samples per pixel) for antialiasing
    aa: u8,                                                                     // Samples per pixel for antialiasing
    dus: Vec3,                                                                  // delta_u between samples
    dvs: Vec3,                                                                  // delta_v between samples
    s_corner: Vec3,                                                             // Position of the first top-left sample
}

impl Camera {
    pub fn new(aspect_ratio: f64, viewport_width: u16, max_depth: u8, focal_length: f64, aa_factor: u8) -> Camera {
        let h: u16 = (viewport_width as f64/aspect_ratio) as u16;
        let vh: f64 = 2.0;
        let vw: f64 = vh*(viewport_width as f64/h as f64);
        let vu: Vec3 = Vec3(vw as f64, 0., 0.);
        let vv: Vec3 = Vec3(0., -vh as f64, 0.);
        let du: Vec3 = vu/(viewport_width as f64);
        let dv: Vec3 = vv/(h as f64);
        let c_center: Vec3 = Vec3(0., 0., 0.);
        let v_corner: Vec3 = c_center - Vec3(0., 0., focal_length) - vu/2. - vv/2.;
        let aa_sqrt: u8 = (aa_factor as f64).sqrt() as u8;
        let aa: u8 = aa_sqrt*aa_sqrt;
        let dus: Vec3 = du/(aa_sqrt + 1) as f64;
        let dvs: Vec3 = dv/(aa_sqrt + 1) as f64;
        let s_corner: Vec3 = v_corner + dus + dvs;
        
        Camera {
            max_d: max_depth,
            ar: aspect_ratio,
            f: focal_length,
            w: viewport_width,
            h: h,
            vh: vh,
            vw: vw,
            c_center: c_center,
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
        println!("P3\n{} {}\n255", self.w, self.h);
        for j in 0..self.h {
            for i in 0..self.w {
                write_color(self.get_px_color(&world, i, j));
            }
        }
    }

    fn get_px_color(&self, world: &World, i: u16, j: u16) -> Vec3 {
        let mut px_color: Vec3 = Vec3(0., 0., 0.);
        let start: Vec3 = self.s_corner + (i as f64)*self.du + (j as f64)*self.dv;
        for pi in 0..self.aa_sqrt {
            for pj in 0..self.aa_sqrt {
                let pos: Vec3 = start + (pi as f64)*self.dus + (pj as f64)*self.dvs;
                let r: Ray = Ray {ori: self.c_center, dir: pos - self.c_center};
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