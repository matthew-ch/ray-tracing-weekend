use std::sync::Arc;

use rand::random;

use crate::{Float, HitRecord, Hittable, Ray, Vec3, AABB};

enum BvhChild<'a> {
    Leaf(Option<&'a dyn Hittable>),
    Node(Arc<BvhNode<'a>>),
}

use BvhChild::*;

impl<'a> BvhChild<'a> {
    fn inner(&'a self) -> Option<&'a dyn Hittable> {
        match self {
            Leaf(ref leaf) => *leaf,
            Node(ref node) => Some(&**node),
        }
    }
}

pub struct BvhNode<'a> {
    bbox: AABB,
    left: BvhChild<'a>,
    right: BvhChild<'a>,
}

impl<'a> BvhNode<'a> {
    pub fn new(src_objects: &[&'a dyn Hittable], time0: Float, time1: Float) -> BvhNode<'a> {
        let mut objects = src_objects.to_owned();
        let axis = random::<usize>() % 3;
        let f = match axis {
            0 => Vec3::x,
            1 => Vec3::y,
            _ => Vec3::z,
        };
        let comparator = |a: &&dyn Hittable, b: &&dyn Hittable| {
            let mut box_a = AABB::default();
            let mut box_b = AABB::default();
            if !a.bounding_box(0.0, 0.0, &mut box_a) || !b.bounding_box(0.0, 0.0, &mut box_b) {
                eprintln!("No bouding box in bvh_node contructor.");
            }
            f(&box_a.min()).partial_cmp(&f(&box_b.min())).unwrap()
        };

        let len = src_objects.len();

        let (left, right): (BvhChild, BvhChild) = match len {
            1 => (Leaf(Some(objects[0])), Leaf(None)),
            2 => (Leaf(Some(objects[0])), Leaf(Some(objects[1]))),
            _ => {
                objects.sort_by(comparator);
                let mid = len / 2;
                let (first, second) = objects.split_at(mid);
                (
                    Node(Arc::new(BvhNode::new(first, time0, time1))),
                    Node(Arc::new(BvhNode::new(second, time0, time1))),
                )
            }
        };

        let mut box_left = AABB::default();
        let mut box_right = AABB::default();
        if !left
            .inner()
            .map_or(false, |left| left.bounding_box(time0, time1, &mut box_left))
            || !right.inner().map_or(false, |right| {
                right.bounding_box(time0, time1, &mut box_right)
            })
        {
            eprintln!("No bouding box in bvh_node constructor.");
        }

        Self {
            bbox: AABB::surrounding_box(box_left, box_right),
            left,
            right,
        }
    }
}

impl Hittable for BvhNode<'_> {
    fn hit<'a, 'b>(&'a self, ray: &Ray, t_min: Float, t_max: Float, rec: &mut HitRecord<'b>) -> bool
    where
        'a: 'b,
    {
        if !self.bbox.hit(ray, t_min, t_max) {
            return false;
        }
        let hit_left = self
            .left
            .inner()
            .map_or(false, |left| left.hit(ray, t_min, t_max, rec));
        let hit_right = self.right.inner().map_or(hit_left, |right| {
            right.hit(ray, t_min, if hit_left { rec.t } else { t_max }, rec)
        });
        hit_left || hit_right
    }

    fn bounding_box(&self, _time0: Float, _time1: Float, output_box: &mut AABB) -> bool {
        *output_box = self.bbox;
        true
    }
}
