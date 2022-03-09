use std::mem;

use crate::{Color, Float, Perlin, Point3};

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

pub struct NoiseTexture {
    noise: Perlin,
    scale: Float,
}

impl NoiseTexture {
    pub fn new(scale: Float) -> Self {
        Self {
            noise: Perlin::new(),
            scale,
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: Float, _v: Float, p: &Point3) -> Color {
        Color::new(1.0, 1.0, 1.0)
            * 0.5
            * (1.0 + (p.z() * self.scale + 10.0 * self.noise.turb(p, 7)).sin())
    }
}

pub struct ImageTexture {
    data: Vec<[u8; 3]>,
    width: usize,
    height: usize,
}

impl ImageTexture {
    const BYTES_PER_PIXEL: usize = 3;

    pub fn new(data: Vec<u8>, width: u32, height: u32) -> Self {
        let mut data = mem::ManuallyDrop::new(data);
        let width = width as usize;
        let height = height as usize;
        assert_eq!(data.len(), width * height * Self::BYTES_PER_PIXEL);
        assert_eq!(mem::align_of::<u8>(), mem::align_of::<[u8; 3]>());
        let new_data = unsafe {
            let ptr = data.as_mut_ptr() as *mut [u8; 3];
            Vec::from_raw_parts(
                ptr,
                data.len() / Self::BYTES_PER_PIXEL,
                data.capacity() / Self::BYTES_PER_PIXEL,
            )
        };

        Self {
            data: new_data,
            width,
            height,
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: Float, v: Float, _p: &Point3) -> Color {
        if self.data.len() == 0 {
            return Color::new(0.0, 1.0, 1.0);
        }
        let u = u.clamp(0.0, 1.0);
        let v = 1.0 - v.clamp(0.0, 1.0);
        let i = ((u * self.width as Float) as usize).min(self.width - 1);
        let j = ((v * self.height as Float) as usize).min(self.height - 1);

        let color_scale = 1.0 / 255.0;
        let pixel = self.data[j * self.width + i];

        Color::new(
            pixel[0] as Float * color_scale,
            pixel[1] as Float * color_scale,
            pixel[2] as Float * color_scale,
        )
    }
}
