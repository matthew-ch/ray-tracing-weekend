use crate::{Float, Point3, Vec3};

#[derive(Debug, Clone, Copy, Default)]
pub struct Ray {
    orig: Vec3,
    dir: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self {
            orig: origin,
            dir: direction,
        }
    }

    pub fn origin(&self) -> Point3 {
        self.orig
    }

    pub fn direction(&self) -> Vec3 {
        self.dir
    }

    pub fn at(&self, t: Float) -> Point3 {
        self.orig + t * self.dir
    }
}
