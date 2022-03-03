use crate::{Float, Material, Point3, Ray, Vec3};

#[derive(Default, Clone)]
pub struct HitRecord<'a> {
    pub p: Point3,
    pub normal: Vec3,
    pub t: Float,
    pub front_face: bool,
    pub material: Option<&'a dyn Material>,
}

impl HitRecord<'_> {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = ray.direction().dot(&outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

pub trait Hittable: Sync + Send {
    fn hit<'a, 'b>(
        &'a self,
        ray: &Ray,
        t_min: Float,
        t_max: Float,
        rec: &mut HitRecord<'b>,
    ) -> bool
    where
        'a: 'b;
}
