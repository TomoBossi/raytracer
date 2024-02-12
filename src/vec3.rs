use crate::random::{random, random_in};

#[derive(Clone, Copy)]
pub struct Vec3(pub f64, pub f64, pub f64);

impl Vec3 {
    pub fn random() -> Vec3 { // Random vector with all coordinates between 1 and -1 
        Vec3(random(), random(), random())
    }

    pub fn random_in(min: f64, max: f64) -> Vec3 { // Random vector with all coordinates in the range defined by min and max
        Vec3(random_in(min, max), random_in(min, max), random_in(min, max))
    }

    pub fn random_in_unit_sphere() -> Vec3 { // Random vector inside the unit sphere
        loop {
            let v: Vec3 = Self::random_in(-1., 1.);
            if (v.l2norm() < 1.) {
                return v;
            }
        }
    }

    pub fn random_in_unit_disk() -> Vec3 { // Random vector on the unit circle
        loop {
            let v: Vec3 = Vec3(random_in(-1., 1.), random_in(-1., 1.), 0.);
            if (v.l2norm() < 1.) {
                return v;
        }
    }

    pub fn random_unit() -> Vec3 { // Random unit vector
        Self::random_in_unit_sphere().unit()
    }

    pub fn random_unit_hemisphere(n: Vec3) -> Vec3 { // Random unit vector facing the hemisphere of the unit sphere derived from the surface normal
        let v: Vec3 = Self::random_unit();
        (if (v*n > 0.) {1.} else {-1.})*v
    }

    pub fn reflect(self, n: Vec3) -> Vec3 { // Reflect self based on surface normal
        self - (2.*self*n)*n
    }

    pub fn refract(self, n: Vec3, idx_ratio: f64) -> Vec3 { // Refract self based on surface normal and refraction index ratio (eta over eta prime)
        let cos_theta: f64 = f64::min(-self*n, 1.);
        let r_out_perpendicular: Vec3 = idx_ratio*(self + cos_theta*n);
        let r_out_parallel: Vec3 = -((1. - r_out_perpendicular.l2norm()).abs().sqrt())*n;
        r_out_perpendicular + r_out_parallel
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

    pub fn near_zero(self) -> bool {
        let atol: f64 = 1e-8;
        (self.0.abs() < atol) && (self.1.abs() < atol) && (self.2.abs() < atol)
    }
}

impl std::fmt::Display for Vec3 { // println!("{}", v);
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {} {}", self.0, self.1, self.2)
    }
}

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