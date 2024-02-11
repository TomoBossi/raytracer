use crate::random::{random, random_in};

pub struct Vec3(pub f64, pub f64, pub f64);

impl Vec3 {
    pub fn random() -> Vec3 {
        Vec3(random(), random(), random())
    }

    pub fn random_in(min: f64, max: f64) -> Vec3 {
        Vec3(random_in(min, max), random_in(min, max), random_in(min, max))
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let v: Vec3 = Self::random_in(-1., 1.);
            if (v.l2norm() < 1.) {
                return v;
            }
        }
    }

    pub fn random_unit() -> Vec3 {
        Self::random_in_unit_sphere().unit()
    }

    pub fn random_unit_hemisphere(n: Vec3) -> Vec3 {
        let v: Vec3 = Self::random_unit();
        (if (v*n > 0.) {1.} else {-1.})*v
    }

    pub fn l2norm(self) -> f64 { // L2 norm
        self.0*self.0 + self.1*self.1 + self.2*self.2
    }

    pub fn len(self) -> f64 { // Euclidean distance
        self.l2norm().sqrt()
    }

    pub fn coord_mul(self, rhs: Vec3) -> Vec3 { // Coordinate-wise product
        Vec3 (
            self.0*rhs.0,
            self.1*rhs.1,
            self.2*rhs.2
        )
    }

    pub fn x(self, rhs: Vec3) -> Vec3 { // Cross/Vector product
        Vec3 (
            self.1*rhs.2 - self.2*rhs.1,
            self.2*rhs.0 - self.0*rhs.2,
            self.0*rhs.1 - self.1*rhs.0
        )
    }

    pub fn unit(self) -> Vec3 { // Normalize
        self/self.len()
    }
}

impl std::fmt::Display for Vec3 { // println!("{}", v);
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {} {}", self.0, self.1, self.2)
    }
}

impl std::clone::Clone for Vec3 { // v.clone();
    fn clone(&self) -> Self {
        Vec3 (
            self.0.clone(),
            self.1.clone(),
            self.2.clone()
        )
    }
}

impl std::marker::Copy for Vec3 {}

impl std::ops::Neg for Vec3 { // -v;
    type Output = Self;
    fn neg(self) -> Self::Output {
        Vec3 (
            -self.0,
            -self.1,
            -self.2
        )
    }
}

impl std::ops::Add for Vec3 { // v1 + v2;
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Vec3 (
            self.0 + rhs.0,
            self.1 + rhs.1,
            self.2 + rhs.2
        )
    }
}

impl std::ops::AddAssign for Vec3 { // v1 += v2;
    fn add_assign(&mut self, rhs: Self) {
        *self = Vec3 (
            self.0 + rhs.0,
            self.1 + rhs.1,
            self.2 + rhs.2
        );
    }
}

impl std::ops::Add<Vec3> for f64 { // Coordinate-wise scalar sum;
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3 (
            self + rhs.0,
            self + rhs.1,
            self + rhs.2
        )
    }
}

impl std::ops::Sub for Vec3 { // v1 - v2;
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 (
            self.0 - rhs.0,
            self.1 - rhs.1,
            self.2 - rhs.2
        )
    }
}

impl std::ops::Mul<Vec3> for f64 { // k*v;
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 (
            self*rhs.0,
            self*rhs.1,
            self*rhs.2
        )
    }
}

impl std::ops::Mul for Vec3 { // Dot product
    type Output = f64;
    fn mul(self, rhs: Vec3) -> Self::Output {
        self.0*rhs.0 + self.1*rhs.1 + self.2*rhs.2
    }
}

impl std::ops::Div<f64> for Vec3 { // v/k;
    type Output = Vec3;
    fn div(self, rhs: f64) -> Self::Output {
        (1./rhs)*self
    }
}