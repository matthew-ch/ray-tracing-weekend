use std::sync::Arc;

use crate::{
    Float, HitRecord, Hittable, HittableList, Material, Point3, Ray, XyRect, XzRect, YzRect, AABB,
};

pub struct BlockBox {
    box_min: Point3,
    box_max: Point3,
    sides: HittableList,
}

impl BlockBox {
    pub fn new(p0: Point3, p1: Point3, material: Arc<dyn Material>) -> Self {
        let mut sides = HittableList::new();

        sides.add(XyRect::new(
            p0.x(),
            p1.x(),
            p0.y(),
            p1.y(),
            p1.z(),
            material.clone(),
        ));
        sides.add(XyRect::new(
            p0.x(),
            p1.x(),
            p0.y(),
            p1.y(),
            p0.z(),
            material.clone(),
        ));

        sides.add(XzRect::new(
            p0.x(),
            p1.x(),
            p0.z(),
            p1.z(),
            p1.y(),
            material.clone(),
        ));
        sides.add(XzRect::new(
            p0.x(),
            p1.x(),
            p0.z(),
            p1.z(),
            p0.y(),
            material.clone(),
        ));

        sides.add(YzRect::new(
            p0.y(),
            p1.y(),
            p0.z(),
            p1.z(),
            p1.x(),
            material.clone(),
        ));
        sides.add(YzRect::new(
            p0.y(),
            p1.y(),
            p0.z(),
            p1.z(),
            p0.x(),
            material,
        ));

        Self {
            box_min: p0,
            box_max: p1,
            sides,
        }
    }
}

impl Hittable for BlockBox {
    fn bounding_box(&self, _time0: Float, _time1: Float, output_box: &mut AABB) -> bool {
        *output_box = AABB::new(self.box_min, self.box_max);
        true
    }

    fn hit<'a, 'b>(&'a self, ray: &Ray, t_min: Float, t_max: Float, rec: &mut HitRecord<'b>) -> bool
    where
        'a: 'b,
    {
        self.sides.hit(ray, t_min, t_max, rec)
    }
}
