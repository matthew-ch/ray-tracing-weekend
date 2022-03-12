use std::sync::Arc;

use crate::{Float, HitRecord, Hittable, Ray, Vec3, AABB};

pub struct Translate {
    inner: Arc<dyn Hittable>,
    offset: Vec3,
}

impl Translate {
    pub fn new(p: Arc<dyn Hittable>, displacement: Vec3) -> Self {
        Self {
            inner: p,
            offset: displacement,
        }
    }
}

impl Hittable for Translate {
    fn hit<'a, 'b>(&'a self, ray: &Ray, t_min: Float, t_max: Float, rec: &mut HitRecord<'b>) -> bool
    where
        'a: 'b,
    {
        let moved_ray = Ray::new(ray.origin() - self.offset, ray.direction(), ray.time());
        if !self.inner.hit(&moved_ray, t_min, t_max, rec) {
            false
        } else {
            rec.p += self.offset;
            rec.set_face_normal(&moved_ray, rec.normal);
            true
        }
    }

    fn bounding_box(&self, time0: Float, time1: Float, output_box: &mut AABB) -> bool {
        if !self.inner.bounding_box(time0, time1, output_box) {
            false
        } else {
            *output_box = AABB::new(
                output_box.min() + self.offset,
                output_box.max() + self.offset,
            );
            true
        }
    }
}
