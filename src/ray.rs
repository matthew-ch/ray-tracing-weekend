use crate::{Float, Point3, Vec3};

#[derive(Debug, Clone, Copy, Default)]
pub struct Ray {
    orig: Vec3,
    dir: Vec3,
    tm: Float,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3, time: Float) -> Self {
        Self {
            orig: origin,
            dir: direction,
            tm: time,
        }
    }

    pub const fn origin(&self) -> Point3 {
        self.orig
    }

    pub const fn direction(&self) -> Vec3 {
        self.dir
    }

    pub const fn time(&self) -> Float {
        self.tm
    }

    pub fn at(&self, t: Float) -> Point3 {
        self.orig + t * self.dir
    }
}
