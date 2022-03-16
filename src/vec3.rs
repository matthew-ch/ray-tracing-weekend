use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Range, Sub, SubAssign};

use rand::random;

use crate::{Float, PI};

#[derive(Debug, Clone, Copy, Default)]
pub struct Vec3(Float, Float, Float);

impl Vec3 {
    pub const fn new(a: Float, b: Float, c: Float) -> Self {
        Self(a, b, c)
    }

    pub fn random() -> Self {
        Self(random(), random(), random())
    }

    pub fn random_range(range: Range<Float>) -> Self {
        let start = range.start;
        let end = range.end;
        Self::new(start, start, start) + Self::random() * (end - start)
    }

    pub fn random_in_unit_sphere() -> Self {
        Self::random_unit_vector() * random::<Float>()
    }

    pub fn random_unit_vector() -> Self {
        let alpha = random::<Float>() * PI;
        let beta = random::<Float>() * PI * 2.0;
        let (a, z) = alpha.sin_cos();
        let (y, x) = beta.sin_cos();

        Self(x * a, y * a, z)
    }

    pub fn random_in_hemisphere(normal: &Self) -> Self {
        let v = Self::random_in_unit_sphere();
        if v.dot(&normal) > 0.0 {
            v
        } else {
            -v
        }
    }

    pub const fn x(&self) -> Float {
        self.0
    }

    pub const fn y(&self) -> Float {
        self.1
    }

    pub const fn z(&self) -> Float {
        self.2
    }

    pub fn length_squared(&self) -> Float {
        self.dot(self)
    }

    pub fn length(&self) -> Float {
        self.dot(self).sqrt()
    }

    pub fn dot(&self, v: &Self) -> Float {
        self.0 * v.0 + self.1 * v.1 + self.2 * v.2
    }

    pub fn cross(&self, v: &Self) -> Self {
        Self(
            self.1 * v.2 - self.2 * v.1,
            self.2 * v.0 - self.0 * v.2,
            self.0 * v.1 - self.1 * v.0,
        )
    }

    pub fn unit_vector(&self) -> Self {
        *self / self.length()
    }

    pub fn apply(&self, f: fn(Float) -> Float) -> Self {
        Self(f(self.0), f(self.1), f(self.2))
    }

    pub fn near_zero(&self) -> bool {
        const S: Float = 1e-8;
        self.0.abs() < S && self.1.abs() < S && self.2.abs() < S
    }

    pub fn reflect(&self, normal: &Self) -> Self {
        *self - 2.0 * self.dot(normal) * *normal
    }

    pub fn refract(&self, normal: &Self, etai_over_etat: Float) -> Self {
        let cos_theta = normal.dot(&-*self).min(1.0);
        let r_out_perp = etai_over_etat * (*self + cos_theta * *normal);
        let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * *normal;
        r_out_perp + r_out_parallel
    }

    pub fn rotate_x(&self, theta: Float) -> Self {
        let (sin, cos) = theta.sin_cos();
        Self(
            self.0,
            cos * self.1 - sin * self.2,
            sin * self.1 - cos * self.2,
        )
    }

    pub fn rotate_y(&self, theta: Float) -> Self {
        let (sin, cos) = theta.sin_cos();
        Self(
            cos * self.0 + sin * self.2,
            self.1,
            -sin * self.0 + cos * self.2,
        )
    }

    pub fn rotate_z(&self, theta: Float) -> Self {
        let (sin, cos) = theta.sin_cos();
        Self(
            cos * self.0 - sin * self.1,
            sin * self.0 + cos * self.1,
            self.2,
        )
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec3(-self.0, -self.1, -self.2)
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
        self.2 -= rhs.2;
    }
}

impl Mul<Float> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Float) -> Self::Output {
        Vec3(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

impl Mul<Vec3> for Float {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs.mul(self)
    }
}

impl MulAssign<Float> for Vec3 {
    fn mul_assign(&mut self, rhs: Float) {
        self.0 *= rhs;
        self.1 *= rhs;
        self.2 *= rhs;
    }
}

impl Div<Float> for Vec3 {
    type Output = Self;

    fn div(self, rhs: Float) -> Self::Output {
        self.mul(1.0 / rhs)
    }
}

impl DivAssign<Float> for Vec3 {
    fn div_assign(&mut self, rhs: Float) {
        self.mul_assign(1.0 / rhs);
    }
}

impl From<Vec3> for [u8; 3] {
    fn from(v: Vec3) -> Self {
        [
            (v.0 * 255.999) as u8,
            (v.1 * 255.999) as u8,
            (v.2 * 255.999) as u8,
        ]
    }
}

impl From<Vec3> for [Float; 3] {
    fn from(v: Vec3) -> Self {
        [v.0, v.1, v.2]
    }
}
