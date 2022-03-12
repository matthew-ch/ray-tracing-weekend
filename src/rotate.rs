use std::sync::Arc;

use crate::{Float, HitRecord, Hittable, Point3, Ray, AABB};

macro_rules! makeRotate {
    ($name: ident, $f: ident) => {
        pub struct $name {
            theta: Float,
            hasbox: bool,
            inner: Arc<dyn Hittable>,
            bbox: AABB,
        }

        impl $name {
            pub fn new(p: Arc<dyn Hittable>, angle: Float) -> Self {
                let theta = angle.to_radians();
                let mut bbox = AABB::default();
                let hasbox = p.bounding_box(0.0, 1.0, &mut bbox);

                let mut vertexes = Vec::new();

                for x in [bbox.min().x(), bbox.max().x()] {
                    for y in [bbox.min().y(), bbox.max().y()] {
                        for z in [bbox.min().z(), bbox.max().z()] {
                            vertexes.push(Point3::new(x, y, z).$f(theta));
                        }
                    }
                }

                let bbox = vertexes
                    .into_iter()
                    .map(|v| AABB::new(v, v))
                    .reduce(AABB::surrounding_box)
                    .unwrap();

                Self {
                    theta,
                    hasbox,
                    inner: p,
                    bbox,
                }
            }
        }

        impl Hittable for $name {
            fn bounding_box(&self, _time0: Float, _time1: Float, output_box: &mut AABB) -> bool {
                *output_box = self.bbox;
                self.hasbox
            }

            fn hit<'a, 'b>(
                &'a self,
                ray: &Ray,
                t_min: Float,
                t_max: Float,
                rec: &mut HitRecord<'b>,
            ) -> bool
            where
                'a: 'b,
            {
                let origin = ray.origin().$f(-self.theta);
                let direction = ray.direction().$f(-self.theta);

                let rotated_ray = Ray::new(origin, direction, ray.time());
                if !self.inner.hit(&rotated_ray, t_min, t_max, rec) {
                    return false;
                }

                let p = rec.p.$f(self.theta);
                let normal = rec.normal.$f(self.theta);

                rec.p = p;
                rec.set_face_normal(&rotated_ray, normal);

                true
            }
        }
    };
}

makeRotate!(RotateX, rotate_x);
makeRotate!(RotateY, rotate_y);
makeRotate!(RotateZ, rotate_z);
