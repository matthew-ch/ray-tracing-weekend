use std::{ops::Deref, sync::Arc};

use rand::random;

use crate::{Float, HitRecord, Hittable, Point3, Ray, Vec3, AABB};

type Item = Arc<dyn Hittable>;

pub struct HittableList {
    objects: Vec<Item>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn new_with(object: impl Hittable + 'static) -> Self {
        let mut list = Self::new();
        list.add(object);
        list
    }

    pub fn add(&mut self, object: impl Hittable + 'static) {
        self.objects.push(Arc::new(object));
    }

    pub fn add_shared(&mut self, object: Item) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit<'a, 'b>(&'a self, ray: &Ray, t_min: Float, t_max: Float, rec: &mut HitRecord<'b>) -> bool
    where
        'a: 'b,
    {
        let mut temp_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            if object.hit(ray, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }

        hit_anything
    }

    fn bounding_box(&self, time0: Float, time1: Float, output_box: &mut AABB) -> bool {
        if self.objects.is_empty() {
            return false;
        }

        let mut temp_box = AABB::default();
        let mut first_box = true;
        for object in &self.objects {
            if !object.bounding_box(time0, time1, &mut temp_box) {
                return false;
            }
            *output_box = if first_box {
                temp_box
            } else {
                AABB::surrounding_box(*output_box, temp_box)
            };
            first_box = false;
        }
        true
    }

    fn pdf_value(&self, o: &Point3, v: &Vec3) -> Float {
        let weight = 1.0 / (self.objects.len() as Float);
        let sum = self
            .objects
            .iter()
            .map(|object| weight * object.pdf_value(o, v))
            .sum();
        sum
    }

    fn random(&self, o: &Vec3) -> Vec3 {
        let idx = random::<usize>() % self.objects.len();
        self.objects[idx].random(o)
    }
}

impl Deref for HittableList {
    type Target = [Item];

    fn deref(&self) -> &Self::Target {
        &self.objects
    }
}
