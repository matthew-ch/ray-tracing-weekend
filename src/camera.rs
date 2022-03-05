use rand::random;

use crate::{Float, Point3, Ray, Vec3, PI};

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    lens_radius: Float,
    time0: Float,
    time1: Float,
}

impl Camera {
    pub fn new(
        lookfrom: Point3,
        lookat: Point3,
        vup: Vec3,
        vfov: Float,
        aspect_ratio: Float,
        aperture: Float,
        focus_dist: Float,
        time0: Float,
        time1: Float,
    ) -> Self {
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();

        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).unit_vector();
        let u = vup.cross(&w).unit_vector();
        let v = w.cross(&u);

        let origin = lookfrom;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;

        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner: origin - horizontal / 2.0 - vertical / 2.0 - focus_dist * w,
            u,
            v,
            lens_radius: aperture / 2.0,
            time0,
            time1,
        }
    }

    pub fn get_ray(&self, s: Float, t: Float) -> Ray {
        let (sin, cos) = (random::<Float>() * PI * 2.0).sin_cos();
        let offset = random::<Float>() * self.lens_radius * (cos * self.u + sin * self.v);

        Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
            random::<Float>() * (self.time1 - self.time0) + self.time0,
        )
    }
}
