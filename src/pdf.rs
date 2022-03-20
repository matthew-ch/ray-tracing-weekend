use rand::random;

use crate::{Float, Hittable, Onb, Point3, Vec3, PI};

pub trait Pdf {
    fn value(&self, direction: &Vec3) -> Float;
    fn generate(&self) -> Vec3;
}

pub struct CosinePdf {
    uvw: Onb,
}

impl CosinePdf {
    pub fn new(w: &Vec3) -> Self {
        Self { uvw: Onb::from(w) }
    }
}

impl Pdf for CosinePdf {
    fn value(&self, direction: &Vec3) -> Float {
        let cosine = direction.unit_vector().dot(&self.uvw.w());
        if cosine < 0.0 {
            0.0
        } else {
            cosine / PI
        }
    }

    fn generate(&self) -> Vec3 {
        self.uvw.local_v(&Vec3::random_cosine_direction())
    }
}

pub struct HittablePdf<'a> {
    o: Point3,
    ptr: &'a dyn Hittable,
}

impl<'a> HittablePdf<'a> {
    pub fn new(p: &'a dyn Hittable, origin: Point3) -> Self {
        Self { o: origin, ptr: p }
    }
}

impl Pdf for HittablePdf<'_> {
    fn value(&self, direction: &Vec3) -> Float {
        self.ptr.pdf_value(&self.o, direction)
    }

    fn generate(&self) -> Vec3 {
        self.ptr.random(&self.o)
    }
}

pub struct MixturePdf<'a> {
    p: [&'a dyn Pdf; 2],
}

impl<'a> MixturePdf<'a> {
    pub fn new(p0: &'a dyn Pdf, p1: &'a dyn Pdf) -> Self {
        Self { p: [p0, p1] }
    }
}

impl Pdf for MixturePdf<'_> {
    fn value(&self, direction: &Vec3) -> Float {
        0.5 * self.p[0].value(direction) + 0.5 * self.p[1].value(direction)
    }

    fn generate(&self) -> Vec3 {
        if random::<Float>() < 0.5 {
            self.p[0].generate()
        } else {
            self.p[1].generate()
        }
    }
}
