mod camera;
mod hittable;
mod hittable_list;
mod material;
mod ray;
mod sphere;
mod vec3;

pub type Float = f64;

pub use camera::Camera;
pub use hittable::*;
pub use hittable_list::HittableList;
pub use material::*;
pub use ray::Ray;
pub use sphere::Sphere;
pub use vec3::Vec3;

pub type Point3 = Vec3;
pub type Color = Vec3;

pub fn hit_sphere(center: &Point3, radius: Float, ray: &Ray) -> Option<Float> {
    let oc = ray.origin() - *center;
    let a = ray.direction().length_squared();
    let half_b = oc.dot(&ray.direction());
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;

    if discriminant < 0.0 {
        None
    } else {
        Some((-half_b - discriminant.sqrt()) / a)
    }
}
