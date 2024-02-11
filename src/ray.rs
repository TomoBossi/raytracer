use crate::vec3::Vec3;

pub struct Ray {
    pub ori: Vec3,
    pub dir: Vec3
}

impl Ray {
    pub fn at(self, t: f64) -> Vec3 {
        self.ori + t*self.dir
    }
}

impl std::clone::Clone for Ray { // r.clone();
    fn clone(&self) -> Self {
        Ray {
            ori: self.ori.clone(),
            dir: self.dir.clone()
        }
    }
}

impl std::marker::Copy for Ray {}