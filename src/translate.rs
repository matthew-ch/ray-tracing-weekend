use crate::{Float, Hittable, Ray, Vec3, AABB};

pub struct Translate {
    inner: Box<dyn Hittable>,
    offset: Vec3,
}

impl Translate {
    pub fn new(p: impl Hittable + 'static, displacement: Vec3) -> Self {
        Self {
            inner: Box::new(p),
            offset: displacement,
        }
    }
}

impl Hittable for Translate {
    fn hit<'a, 'b>(
        &'a self,
        ray: &crate::Ray,
        t_min: Float,
        t_max: Float,
        rec: &mut crate::HitRecord<'b>,
    ) -> bool
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
