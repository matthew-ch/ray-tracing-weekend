use std::rc::Rc;

use crate::{Float, HitRecord, Hittable, Material, Point3, Ray};

pub struct Sphere {
    center: Point3,
    radius: Float,
    material: Option<Rc<Box<dyn Material>>>,
}

impl Sphere {
    pub fn new(center: Point3, radius: Float, material: Option<Rc<Box<dyn Material>>>) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: Float, t_max: Float, rec: &mut HitRecord) -> bool {
        let oc = ray.origin() - self.center;
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
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(ray, outward_normal);
        rec.material = self.material.clone();

        true
    }
}
