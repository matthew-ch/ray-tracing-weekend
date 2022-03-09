use crate::{Float, HitRecord, Hittable, Material, Point3, Ray, Vec3, AABB, PI};

pub struct Sphere {
    center: Point3,
    radius: Float,
    material: Option<Box<dyn Material>>,
}

impl Sphere {
    pub fn new(center: Point3, radius: Float, material: Option<impl Material + 'static>) -> Self {
        Self {
            center,
            radius,
            material: material.map(|m| Box::new(m) as Box<dyn Material>),
        }
    }

    pub fn get_uv(p: &Point3, u: &mut Float, v: &mut Float) {
        let theta = (-p.y()).acos();
        let phi = (-p.z()).atan2(p.x()) + PI;

        *u = phi / (2.0 * PI);
        *v = theta / PI;
    }
}

impl Hittable for Sphere {
    fn hit<'a, 'b>(&'a self, ray: &Ray, t_min: Float, t_max: Float, rec: &mut HitRecord<'b>) -> bool
    where
        'a: 'b,
    {
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
        Self::get_uv(&outward_normal, &mut rec.u, &mut rec.v);
        rec.material = self.material.as_deref();

        true
    }

    fn bounding_box(&self, _time0: Float, _time1: Float, output_box: &mut AABB) -> bool {
        *output_box = AABB::new(
            self.center - Vec3::new(self.radius, self.radius, self.radius),
            self.center + Vec3::new(self.radius, self.radius, self.radius),
        );
        true
    }
}
