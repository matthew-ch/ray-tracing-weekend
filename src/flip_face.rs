use std::sync::Arc;

use crate::{Float, HitRecord, Hittable, AABB};

pub struct FlipFace {
    ptr: Arc<dyn Hittable>,
}

impl FlipFace {
    pub fn new(p: Arc<dyn Hittable>) -> Self {
        Self { ptr: p }
    }
}

impl Hittable for FlipFace {
    fn hit<'a, 'b>(
        &'a self,
        ray: &crate::Ray,
        t_min: Float,
        t_max: Float,
        rec: &mut HitRecord<'b>,
    ) -> bool
    where
        'a: 'b,
    {
        if !self.ptr.hit(ray, t_min, t_max, rec) {
            false
        } else {
            rec.front_face = !rec.front_face;
            true
        }
    }

    fn bounding_box(&self, time0: Float, time1: Float, output_box: &mut AABB) -> bool {
        self.ptr.bounding_box(time0, time1, output_box)
    }
}
