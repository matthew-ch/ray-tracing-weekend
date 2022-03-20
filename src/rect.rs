use crate::{random_range, Float, HitRecord, Hittable, Material, Point3, Ray, Vec3, AABB};
use std::sync::Arc;

macro_rules! makeRect {
    ($name: ident, $a: ident, $a0: ident, $a1: ident, $b: ident, $b0: ident, $b1: ident, $c: ident, $p: ident, $on: expr) => {
        pub struct $name {
            material: Arc<dyn Material>,
            $a0: Float,
            $a1: Float,
            $b0: Float,
            $b1: Float,
            k: Float,
        }

        impl $name {
            pub fn new(
                $a0: Float,
                $a1: Float,
                $b0: Float,
                $b1: Float,
                k: Float,
                material: Arc<dyn Material>,
            ) -> Self {
                Self {
                    $a0,
                    $a1,
                    $b0,
                    $b1,
                    k,
                    material,
                }
            }
        }

        impl Hittable for $name {
            fn bounding_box(&self, _time0: Float, _time1: Float, output_box: &mut AABB) -> bool {
                *output_box = AABB::new(
                    $p(self.$a0, self.$b0, self.k - 0.0001),
                    $p(self.$a1, self.$b1, self.k + 0.0001),
                );
                true
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
                let t = (self.k - ray.origin().$c()) / ray.direction().$c();
                if t < t_min || t > t_max {
                    return false;
                }
                let $a = ray.origin().$a() + t * ray.direction().$a();
                let $b = ray.origin().$b() + t * ray.direction().$b();
                if $a < self.$a0 || $a > self.$a1 || $b < self.$b0 || $b > self.$b1 {
                    return false;
                }
                rec.u = ($a - self.$a0) / (self.$a1 - self.$a0);
                rec.v = ($b - self.$b0) / (self.$b1 - self.$b0);
                rec.t = t;
                let outward_normal = $on;
                rec.set_face_normal(ray, outward_normal);
                rec.material = Some(&*self.material);
                rec.p = ray.at(t);
                true
            }

            fn pdf_value(&self, o: &Point3, v: &Vec3) -> Float {
                let mut rec = HitRecord::default();
                if !self.hit(&Ray::new(*o, *v, 0.0), 0.001, Float::INFINITY, &mut rec) {
                    return 0.0;
                }
                let area = (self.$a1 - self.$a0) * (self.$b1 - self.$b0);
                let distance_squared = rec.t * rec.t * v.length_squared();
                let cosine = (v.dot(&rec.normal) / v.length()).abs();
                distance_squared / (cosine * area)
            }

            fn random(&self, o: &Point3) -> Vec3 {
                let random_point = $p(
                    random_range(self.$a0..self.$a1),
                    random_range(self.$b0..self.$b1),
                    self.k,
                );
                random_point - *o
            }
        }
    };
}

#[inline]
fn xy(x: Float, y: Float, k: Float) -> Point3 {
    Point3::new(x, y, k)
}

makeRect!(
    XyRect,
    x,
    x0,
    x1,
    y,
    y0,
    y1,
    z,
    xy,
    Vec3::new(0.0, 0.0, 1.0)
);

#[inline]
fn xz(x: Float, z: Float, k: Float) -> Point3 {
    Point3::new(x, k, z)
}

makeRect!(
    XzRect,
    x,
    x0,
    x1,
    z,
    z0,
    z1,
    y,
    xz,
    Vec3::new(0.0, 1.0, 0.0)
);

#[inline]
fn yz(y: Float, z: Float, k: Float) -> Point3 {
    Point3::new(k, y, z)
}

makeRect!(
    YzRect,
    y,
    y0,
    x1,
    z,
    z0,
    z1,
    x,
    yz,
    Vec3::new(1.0, 0.0, 0.0)
);
