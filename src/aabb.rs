use std::mem::swap;

use crate::{Float, Point3, Ray};

#[derive(Clone, Copy, Default)]
pub struct AABB {
    minimum: Point3,
    maximum: Point3,
}

impl AABB {
    pub fn new(a: Point3, b: Point3) -> Self {
        Self {
            minimum: a,
            maximum: b,
        }
    }

    pub fn surrounding_box(box0: Self, box1: Self) -> Self {
        let small = Point3::new(
            box0.minimum.x().min(box1.minimum.x()),
            box0.minimum.y().min(box1.minimum.y()),
            box0.minimum.z().min(box1.minimum.z()),
        );

        let big = Point3::new(
            box0.maximum.x().max(box1.maximum.x()),
            box0.maximum.y().max(box1.maximum.y()),
            box0.maximum.z().max(box1.maximum.z()),
        );

        Self {
            minimum: small,
            maximum: big,
        }
    }

    pub const fn min(&self) -> Point3 {
        self.minimum
    }

    pub const fn max(&self) -> Point3 {
        self.maximum
    }

    pub fn hit(&self, ray: &Ray, t_min: Float, t_max: Float) -> bool {
        let dir: [Float; 3] = ray.direction().into();
        let ori: [Float; 3] = ray.origin().into();
        let min: [Float; 3] = self.minimum.into();
        let max: [Float; 3] = self.maximum.into();
        for i in 0..3 {
            let inv_d = 1.0 / dir[i];
            let mut t0 = (min[i] - ori[i]) * inv_d;
            let mut t1 = (max[i] - ori[i]) * inv_d;
            if inv_d < 0.0 {
                swap(&mut t0, &mut t1);
            }
            let t_min = if t0 > t_min { t0 } else { t_min };
            let t_max = if t1 < t_max { t1 } else { t_max };
            if t_max <= t_min {
                return false;
            }
        }
        true
    }
}
