use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::color::write_color;
use crate::surface::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::world::World;

pub struct Camera {
    aspect_ratio: f32,                                                          // Aspect ratio
    focal_len: f32,                                                             // Focal length
    w: u16,                                                                     // Screen width
    h: u16,                                                                     // Screen height
    vh: f32,                                                                    // Viewport width
    vw: f32,                                                                    // Viewport height
    cam_center: Vec3,                                                           // Camera center
    vu: Vec3,                                                                   // V_u
    vv: Vec3,                                                                   // V_v
    du: Vec3,                                                                   // delta_u
    dv: Vec3,                                                                   // delta_v
    v_corner: Vec3,                                                             // Viewport upper-left corner location
    px00_loc: Vec3                                                              // Pixel (0, 0) location
}

impl Camera {
    pub fn new(aspect_ratio: f32, focal_length: f32, width: u16) -> Camera {
        let h: u16 = (width as f32/aspect_ratio) as u16;
        let vh: f32 = 2.0;
        let vw: f32 = vh*(width as f32/h as f32);
        let cam_center: Vec3 = Vec3(0., 0., 0.);
        let vu: Vec3 = Vec3(vw as f32, 0., 0.);
        let vv: Vec3 = Vec3(0., -vh as f32, 0.);
        let du: Vec3 = vu/(width as f32);
        let dv: Vec3 = vv/(h as f32);
        let v_corner: Vec3 = cam_center - Vec3(0., 0., focal_length) - vu/2. - vv/2.;
        let px00_loc: Vec3 = v_corner + (du + dv)/2.;
        
        Camera {
            aspect_ratio: aspect_ratio,
            focal_len: focal_length,
            w: width,
            h: h,
            vh: vh,
            vw: vw,
            cam_center: cam_center,
            vu: vu,
            vv: vv,
            du: du,
            dv: dv,
            v_corner: v_corner,
            px00_loc: px00_loc
        }
    }

    fn ray_color(r: Ray, world: &World) -> Vec3 {
        let mut rec: HitRecord = HitRecord::new_empty();
        if (world.hit(r, Interval{min: 0., max: f32::INFINITY}, &mut rec)) {
            (1. + rec.n)/2.
        } else {
            let unit_dir: Vec3 = r.dir.unit();
            let a: f32 = 0.5*(unit_dir.1 + 1.0);
            (1.0 - a)*Vec3(1.0, 1.0, 1.0) + a*Vec3(0.5, 0.7, 1.0)
        }
    }

    pub fn render(self, world: World) {
        let mut px_center: Vec3;
        let mut px_color: Vec3;
        let mut ray_dir: Vec3;
        let mut r: Ray;
        
        println!("P3\n{} {}\n255", self.w, self.h);
        for j in 0..self.h {
            for i in 0..self.w {
                px_center = self.px00_loc + (i as f32)*self.du + (j as f32)*self.dv;
                ray_dir = px_center - self.cam_center;
                r = Ray{ori: self.cam_center, dir: ray_dir};
                px_color = Self::ray_color(r, &world);
                write_color(px_color);
            }
        }
    }
}