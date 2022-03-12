use std::sync::Arc;

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

pub struct Lambertian {
    albedo: Arc<dyn Texture>,
}

impl Lambertian {
    pub fn new_with_color(color: Color) -> Self {
        Self {
            albedo: Arc::new(SolidColor::from(color)),
        }
    }

    pub fn new_with_texture(t: impl Texture + 'static) -> Self {
        Self {
            albedo: Arc::new(t),
        }
    }

    pub fn new_with_shared_texture(t: Arc<dyn Texture>) -> Self {
        Self { albedo: t }
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
    emit: Arc<dyn Texture>,
}

impl DiffuseLight {
    pub fn new_with_color(color: Color) -> Self {
        Self {
            emit: Arc::new(SolidColor::from(color)),
        }
    }

    pub fn new_with_texture(texture: Arc<dyn Texture>) -> Self {
        Self { emit: texture }
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

pub struct Isotropic {
    albedo: Arc<dyn Texture>,
}

impl Isotropic {
    pub fn new_with_color(color: Color) -> Self {
        Self {
            albedo: Arc::new(SolidColor::from(color)),
        }
    }

    pub fn new_with_texture(texture: Arc<dyn Texture>) -> Self {
        Self { albedo: texture }
    }
}

impl Material for Isotropic {
    fn scatter(
        &self,
        ray_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *scattered = Ray::new(rec.p, Vec3::random_in_unit_sphere(), ray_in.time());
        *attenuation = self.albedo.value(rec.u, rec.v, &rec.p);
        true
    }
}
