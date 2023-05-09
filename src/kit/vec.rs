use crate::kit::{Float2, Float3};
use {
    crate::kit::Float4,
    std::{
        fmt::{Debug, Display, Formatter},
        ops::*,
    },
};

#[derive(Copy, Clone)]
pub struct Vec2<T>(T, T)
where
    T: Add + AddAssign + Sub + SubAssign + Mul + MulAssign + Copy + Display;

pub struct Vec3<T>(T, T, T)
where
    T: Add + AddAssign + Sub + SubAssign + Mul + MulAssign + Copy + Display;

pub struct Vec4<T>(T, T, T, T)
where
    T: Add + AddAssign + Sub + SubAssign + Mul + MulAssign + Copy + Display;

/***************************** Float4 *****************************/

impl Float4 {
    pub fn new(x: f64, y: f64, z: f64, a: f64) -> Self {
        Self(x, y, z, a)
    }
}

impl Mul<f64> for Float4 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs, self.2 * rhs, self.3 * rhs)
    }
}

impl Mul<i32> for Float4 {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Self(
            self.0 * rhs as f64,
            self.1 * rhs as f64,
            self.2 * rhs as f64,
            self.3 * rhs as f64,
        )
    }
}

impl MulAssign<Self> for Float4 {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
        self.3 += rhs.3;
    }
}

/***************************** Float3 *****************************/

impl Float3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self(x, y, z)
    }
}

impl Mul<f64> for Float3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl Mul<i32> for Float3 {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Self(
            self.0 * rhs as f64,
            self.1 * rhs as f64,
            self.2 * rhs as f64,
        )
    }
}

impl MulAssign<Self> for Float3 {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

/***************************** Float2 *****************************/

impl Float2 {
    pub fn new(x: f64, y: f64) -> Self {
        Self(x, y)
    }
}

impl Mul<f64> for Float2 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs)
    }
}

impl Mul<i32> for Float2 {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Self(self.0 * rhs as f64, self.1 * rhs as f64)
    }
}

impl MulAssign<Self> for Float2 {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0;
        self.1 *= rhs.1;
    }
}

/***************************** Vec[2,3,4] Debug *****************************/

impl<T> Debug for Vec4<T>
where
    T: Add + AddAssign + Sub + SubAssign + Mul + MulAssign + Copy + Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "Vec4({}, {}, {}, {})",
            self.0, self.1, self.2, self.3
        ))
    }
}

impl<T> Debug for Vec3<T>
where
    T: Add + AddAssign + Sub + SubAssign + Mul + MulAssign + Copy + Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Vec3({}, {}, {})", self.0, self.1, self.2))
    }
}

impl<T> Debug for Vec2<T>
where
    T: Add + AddAssign + Sub + SubAssign + Mul + MulAssign + Copy + Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Vec2({}, {})", self.0, self.1))
    }
}
