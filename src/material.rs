use crate::{Color, Float, HitRecord, Point3, Ray, SolidColor, Texture, Vec3};
use rand::random;

pub trait Material: Sync + Send {
    fn scatter(
        &self,
        ray_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;

    fn emitted(&self, _u: Float, _v: Float, _p: &Point3) -> Color {
        Color::default()
    }
}

pub enum UnsafeMaterialWrapper<T: Material> {
    Owner(*mut T),
    Borrower(*const T),
}

unsafe impl<T: Material> Send for UnsafeMaterialWrapper<T> {}

unsafe impl<T: Material> Sync for UnsafeMaterialWrapper<T> {}

impl<T: Material> UnsafeMaterialWrapper<T> {
    pub fn new(material: T) -> Self {
        let boxed = Box::new(material);
        Self::Owner(Box::into_raw(boxed))
    }

    pub fn borrow(other: &Self) -> Self {
        match other {
            UnsafeMaterialWrapper::Owner(m) => Self::Borrower(*m),
            UnsafeMaterialWrapper::Borrower(n) => Self::Borrower(*n),
        }
    }
}

impl<T: Material> Clone for UnsafeMaterialWrapper<T> {
    fn clone(&self) -> Self {
        Self::borrow(self)
    }
}

impl<T: Material> Drop for UnsafeMaterialWrapper<T> {
    fn drop(&mut self) {
        match self {
            UnsafeMaterialWrapper::Owner(m) => unsafe { drop(Box::from_raw(*m)) },
            UnsafeMaterialWrapper::Borrower(_) => {}
        }
    }
}

impl<T: Material> Material for UnsafeMaterialWrapper<T> {
    fn emitted(&self, u: Float, v: Float, p: &Point3) -> Color {
        match self {
            UnsafeMaterialWrapper::Owner(m) => unsafe { (**m).emitted(u, v, p) },
            UnsafeMaterialWrapper::Borrower(n) => unsafe { (**n).emitted(u, v, p) },
        }
    }

    fn scatter(
        &self,
        ray_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        match self {
            UnsafeMaterialWrapper::Owner(m) => unsafe {
                (**m).scatter(ray_in, rec, attenuation, scattered)
            },
            UnsafeMaterialWrapper::Borrower(n) => unsafe {
                (**n).scatter(ray_in, rec, attenuation, scattered)
            },
        }
    }
}

pub struct Lambertian {
    albedo: Box<dyn Texture>,
}

impl Lambertian {
    pub fn new_with_color(color: Color) -> Self {
        Self {
            albedo: Box::new(SolidColor::from(color)),
        }
    }

    pub fn new_with_texture(t: impl Texture + 'static) -> Self {
        Self {
            albedo: Box::new(t),
        }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        ray_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        *scattered = Ray::new(rec.p, scatter_direction, ray_in.time());
        *attenuation = self.albedo.value(rec.u, rec.v, &rec.p);
        true
    }
}

#[derive(Clone, Copy)]
pub struct Metal {
    albedo: Color,
    fuzz: Float,
}

impl Metal {
    pub fn new(color: Color, f: Float) -> Self {
        Self {
            albedo: color,
            fuzz: if f < 1.0 { f } else { 1.0 },
        }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        ray_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = ray_in.direction().unit_vector().reflect(&rec.normal);
        *scattered = Ray::new(
            rec.p,
            reflected + self.fuzz * Vec3::random_in_unit_sphere(),
            ray_in.time(),
        );
        *attenuation = self.albedo;
        scattered.direction().dot(&rec.normal) > 0.0
    }
}

#[derive(Clone, Copy)]
pub struct Dielectric {
    ir: Float,
}

impl Dielectric {
    pub fn new(index_of_refraction: Float) -> Self {
        Self {
            ir: index_of_refraction,
        }
    }

    fn reflectance(cosine: Float, ref_idx: Float) -> Float {
        let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        let r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        ray_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Color::new(1.0, 1.0, 1.0);
        let refraction_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };
        let unit_direction = ray_in.direction().unit_vector();
        let cos_theta = rec.normal.dot(&-unit_direction).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        let direction =
            if cannot_refract || Self::reflectance(cos_theta, refraction_ratio) > random() {
                unit_direction.reflect(&rec.normal)
            } else {
                unit_direction.refract(&rec.normal, refraction_ratio)
            };
        *scattered = Ray::new(rec.p, direction, ray_in.time());
        true
    }
}

pub struct DiffuseLight {
    emit: Box<dyn Texture>,
}

impl DiffuseLight {
    pub fn new_with_color(color: Color) -> Self {
        Self {
            emit: Box::new(SolidColor::from(color)),
        }
    }

    pub fn new_with_texture(texture: impl Texture + 'static) -> Self {
        Self {
            emit: Box::new(texture),
        }
    }
}

impl Material for DiffuseLight {
    fn scatter(
        &self,
        _ray_in: &Ray,
        _rec: &HitRecord,
        _attenuation: &mut Color,
        _scattered: &mut Ray,
    ) -> bool {
        false
    }

    fn emitted(&self, u: Float, v: Float, p: &Point3) -> Color {
        self.emit.value(u, v, p)
    }
}
