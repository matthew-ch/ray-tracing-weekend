mod ray;
mod vec3;

pub type Float = f64;

pub use ray::Ray;
pub use vec3::Vec3;

pub type Point3 = Vec3;
pub type Color = Vec3;

pub fn hit_sphere(center: &Point3, radius: Float, ray: &Ray) -> Option<Float> {
    let oc = ray.origin() - *center;
    let a = ray.direction().length_squared();
    let b = 2.0 * oc.dot(&ray.direction());
    let c = oc.length_squared() - radius * radius;
    let discriminant = b * b - 4.0 * a * c;

    if discriminant < 0.0 {
        None
    } else {
        Some((-b - discriminant.sqrt()) / (2.0 * a))
    }
}