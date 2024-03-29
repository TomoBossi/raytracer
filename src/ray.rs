use crate::vec3::Vec3;

#[derive(Clone, Copy)]
pub struct Ray {
    pub ori: Vec3,
    pub dir: Vec3
}

impl Ray {
    pub fn at(self, t: f64) -> Vec3 {
        self.ori + t*self.dir
    }
}