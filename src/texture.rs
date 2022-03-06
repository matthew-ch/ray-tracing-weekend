use crate::{Color, Float, Point3};

pub trait Texture: Sync + Send {
    fn value(&self, u: Float, v: Float, p: &Point3) -> Color;
}

pub struct SolidColor(Color);

impl SolidColor {
    pub fn new(red: Float, green: Float, blue: Float) -> Self {
        Self(Color::new(red, green, blue))
    }
}

impl From<Color> for SolidColor {
    fn from(color: Color) -> Self {
        Self(color)
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: Float, _v: Float, _p: &Point3) -> Color {
        self.0
    }
}

pub struct CheckerTexture {
    odd: Box<dyn Texture>,
    even: Box<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(even: Box<dyn Texture>, odd: Box<dyn Texture>) -> Self {
        Self { even, odd }
    }

    pub fn new_with_color(c1: Color, c2: Color) -> Self {
        Self::new(
            Box::new(SolidColor::from(c1)),
            Box::new(SolidColor::from(c2)),
        )
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: Float, v: Float, p: &Point3) -> Color {
        let sines = (10.0 * p.x()).sin() * (10.0 * p.y()).sin() * (10.0 * p.z()).sin();
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}
