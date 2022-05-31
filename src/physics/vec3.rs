use std::ops::*;

#[derive(Debug, Copy, Clone)]
pub struct Vec3(f64, f64, f64);

impl Vec3 {
    pub fn x(&self) -> f64 {
        self.0
    }
    pub fn y(&self) -> f64 {
        self.1
    }
    pub fn z(&self) -> f64 {
        self.2
    }
    pub fn xyz(&self) -> (f64, f64, f64) {
        (self.x(), self.y(), self.z())
    }

    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self(x, y, z)
    }

    pub fn normalize(&self) -> Self {
        self.clone() / self.len()
    }

    pub fn len(&self) -> f64 {
        self.clone() * self.clone()
    }
}

fn multi(v1: Vec3, v2: Vec3) -> f64 {
    v1 * v2
}

// V * F = Vec3
impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self(self.x() * rhs, self.y() * rhs, self.z() * rhs)
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = f64;

    fn mul(self, rhs: Vec3) -> Self::Output {
        (self.0 * rhs.0) + (self.1 * rhs.1) + (self.2 * rhs.2)
    }
}

impl MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.x();
        self.1 *= rhs.y();
        self.2 *= rhs.z();
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self(self.x() / rhs, self.y() / rhs, self.z() / rhs)
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Self;

    fn add(self, rhs: Vec3) -> Self::Output {
        Self(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
    }
}

impl AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.0 += rhs.x();
        self.1 += rhs.y();
        self.2 += rhs.z();
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Self(0.0, 0.0, 0.0)
    }
}
