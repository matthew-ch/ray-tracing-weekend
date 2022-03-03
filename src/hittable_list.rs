use crate::{Float, HitRecord, Hittable, Ray};

type Item = Box<dyn Hittable>;

pub struct HittableList {
    objects: Vec<Item>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn new_with(object: Item) -> Self {
        let mut list = Self::new();
        list.add(object);
        list
    }

    pub fn add(&mut self, object: Item) {
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
}
