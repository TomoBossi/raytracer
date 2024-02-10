use crate::vec3::Vec3;

pub struct Ray {
    pub ori: Vec3,
    pub dir: Vec3
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray {
            ori: origin,
            dir: direction
        }
    }

    pub fn at(self, t: f32) -> Vec3 {
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