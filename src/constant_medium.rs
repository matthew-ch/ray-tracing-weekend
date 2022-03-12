use std::sync::Arc;

use rand::random;

use crate::{Float, HitRecord, Hittable, Isotropic, Material, Ray, Texture, Vec3, AABB};

pub struct ConstantMedium {
    boundary: Arc<dyn Hittable>,
    phase_function: Arc<dyn Material>,
    neg_inv_density: Float,
}

impl ConstantMedium {
    pub fn new(boundary: Arc<dyn Hittable>, density: Float, texture: Arc<dyn Texture>) -> Self {
        Self {
            boundary,
            neg_inv_density: -1.0 / density,
            phase_function: Arc::new(Isotropic::new_with_texture(texture)),
        }
    }
}

impl Hittable for ConstantMedium {
    fn bounding_box(&self, time0: Float, time1: Float, output_box: &mut AABB) -> bool {
        self.boundary.bounding_box(time0, time1, output_box)
    }

    fn hit<'a, 'b>(&'a self, ray: &Ray, t_min: Float, t_max: Float, rec: &mut HitRecord<'b>) -> bool
    where
        'a: 'b,
    {
        let mut rec1 = HitRecord::default();
        let mut rec2 = HitRecord::default();

        if !self
            .boundary
            .hit(ray, -Float::INFINITY, Float::INFINITY, &mut rec1)
        {
            return false;
        }

        if !self
            .boundary
            .hit(ray, rec1.t + 0.0001, Float::INFINITY, &mut rec2)
        {
            return false;
        }

        if rec1.t < t_min {
            rec1.t = t_min;
        }
        if rec2.t > t_max {
            rec2.t = t_max;
        }
        if rec1.t >= rec2.t {
            return false;
        }

        if rec1.t < 0.0 {
            rec1.t = 0.0;
        }

        let ray_length = ray.direction().length();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * random::<Float>().ln();

        if hit_distance > distance_inside_boundary {
            return false;
        }

        rec.t = rec1.t + hit_distance / ray_length;
        rec.p = ray.at(rec.t);

        rec.normal = Vec3::new(1.0, 0.0, 0.0);
        rec.front_face = true;
        rec.material = Some(&*self.phase_function);

        true
    }
}
