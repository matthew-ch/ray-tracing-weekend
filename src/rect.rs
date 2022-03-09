use crate::{Float, HitRecord, Hittable, Material, Point3, Ray, Vec3, AABB};

macro_rules! makeRect {
    ($name: ident, $a: ident, $a0: ident, $a1: ident, $b: ident, $b0: ident, $b1: ident, $c: ident, $p0: ident, $p1: ident, $on: expr) => {
        pub struct $name {
            material: Option<Box<dyn Material>>,
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
                material: Option<impl Material + 'static>,
            ) -> Self {
                Self {
                    $a0,
                    $a1,
                    $b0,
                    $b1,
                    k,
                    material: material.map(|m| Box::new(m) as Box<dyn Material>),
                }
            }
        }

        impl Hittable for $name {
            fn bounding_box(&self, _time0: Float, _time1: Float, output_box: &mut AABB) -> bool {
                *output_box = AABB::new(
                    $p0(self.$a0, self.$b0, self.k),
                    $p1(self.$a1, self.$b1, self.k),
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
                rec.material = self.material.as_deref();
                rec.p = ray.at(t);
                true
            }
        }
    };
}

#[inline]
fn xy0(x: Float, y: Float, k: Float) -> Point3 {
    Point3::new(x, y, k - 0.0001)
}

#[inline]
fn xy1(x: Float, y: Float, k: Float) -> Point3 {
    Point3::new(x, y, k + 0.0001)
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
    xy0,
    xy1,
    Vec3::new(0.0, 0.0, 1.0)
);

#[inline]
fn xz0(x: Float, z: Float, k: Float) -> Point3 {
    Point3::new(x, k - 0.0001, z)
}

#[inline]
fn xz1(x: Float, z: Float, k: Float) -> Point3 {
    Point3::new(x, k + 0.0001, z)
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
    xz0,
    xz1,
    Vec3::new(0.0, 1.0, 0.0)
);

#[inline]
fn yz0(y: Float, z: Float, k: Float) -> Point3 {
    Point3::new(k - 0.0001, y, z)
}

#[inline]
fn yz1(y: Float, z: Float, k: Float) -> Point3 {
    Point3::new(k + 0.0001, y, z)
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
    yz0,
    yz1,
    Vec3::new(1.0, 0.0, 0.0)
);
