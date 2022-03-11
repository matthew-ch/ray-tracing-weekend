use std::sync::Arc;

use crate::{Float, HitRecord, Hittable, Material, Point3, Ray, Vec3, AABB};

pub struct MovingSphere {
    center0: Point3,
    center1: Point3,
    time0: Float,
    time1: Float,
    radius: Float,
    material: Arc<dyn Material>,
}

impl MovingSphere {
    pub fn new(
        center0: Point3,
        center1: Point3,
        time0: Float,
        time1: Float,
        radius: Float,
        material: Arc<dyn Material>,
    ) -> Self {
        Self {
            center0,
            center1,
            time0,
            time1,
            radius,
            material,
        }
    }

    pub fn center(&self, time: Float) -> Point3 {
        self.center0
            + (time - self.time0) / (self.time1 - self.time0) * (self.center1 - self.center0)
    }
}

impl Hittable for MovingSphere {
    fn hit<'a, 'b>(&'a self, ray: &Ray, t_min: Float, t_max: Float, rec: &mut HitRecord<'b>) -> bool
    where
        'a: 'b,
    {
        let oc = ray.origin() - self.center(ray.time());
        let a = ray.direction().length_squared();
        let half_b = oc.dot(&ray.direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }

        rec.t = root;
        rec.p = ray.at(root);
        let outward_normal = (rec.p - self.center(ray.time())) / self.radius;
        rec.set_face_normal(ray, outward_normal);
        rec.material = Some(&*self.material);

        true
    }

    fn bounding_box(&self, time0: Float, time1: Float, output_box: &mut crate::AABB) -> bool {
        *output_box = AABB::surrounding_box(
            AABB::new(
                self.center(time0) - Vec3::new(self.radius, self.radius, self.radius),
                self.center(time0) + Vec3::new(self.radius, self.radius, self.radius),
            ),
            AABB::new(
                self.center(time1) - Vec3::new(self.radius, self.radius, self.radius),
                self.center(time1) + Vec3::new(self.radius, self.radius, self.radius),
            ),
        );
        true
    }
}
