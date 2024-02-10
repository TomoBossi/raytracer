use crate::vec3::Vec3;

pub struct Ray {
    pub ori: Vec3,
    pub dir: Vec3
}

impl Ray {
    fn new(ori: Vec3, dir: Vec3) -> Ray {
        Ray {
            ori: ori,
            dir: dir
        }
    }

    fn at(self, t: f32) -> Vec3 {
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