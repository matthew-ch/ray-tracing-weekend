mod aabb;
mod block_box;
mod bvh_node;
mod camera;
mod constant_medium;
mod hittable;
mod hittable_list;
mod material;
mod moving_sphere;
mod onb;
mod perlin;
mod ray;
mod rect;
mod rotate;
mod sphere;
mod texture;
mod translate;
mod vec3;

pub type Float = f64;
pub use std::f64::consts::PI;

pub use aabb::AABB;
pub use block_box::BlockBox;
pub use bvh_node::BvhNode;
pub use camera::Camera;
pub use constant_medium::ConstantMedium;
pub use hittable::*;
pub use hittable_list::HittableList;
pub use material::*;
pub use moving_sphere::MovingSphere;
pub use onb::Onb;
pub use perlin::Perlin;
use rand::random;
pub use ray::Ray;
pub use rect::*;
pub use rotate::*;
pub use sphere::Sphere;
pub use texture::*;
pub use translate::Translate;
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

pub fn ray_color<'a>(ray: &Ray, background: &Color, world: &'a impl Hittable, depth: i32) -> Color {
    if depth <= 0 {
        return Color::default();
    }
    let mut rec = HitRecord::default();
    if !world.hit(ray, 0.001, f64::INFINITY, &mut rec) {
        return background.clone();
    }
    let mut scattered = Ray::default();
    let mut attenuation = Color::default();
    let emitted = rec
        .material
        .map_or(Color::default(), |m| m.emitted(rec.u, rec.v, &rec.p));

    let mut pdf: Float = 0.0;
    if !rec.material.map_or(false, |mat| {
        mat.scatter(ray, &mut rec, &mut attenuation, &mut scattered, &mut pdf)
    }) {
        emitted
    } else {
        emitted
            + attenuation
                * rec.material.unwrap().scattering_pdf(ray, &rec, &scattered)
                * ray_color(&scattered, background, world, depth - 1)
                / pdf
    }
}

pub fn render<'a>(
    world: &'a impl Hittable,
    background: Color,
    cam: Camera,
    image_width: u32,
    image_height: u32,
    samples_per_pixel: u32,
    max_depth: i32,
) -> Vec<Color> {
    println!("begin render");
    let mut image: Vec<Color> = Vec::with_capacity((image_width * image_height) as usize);

    for j in (0..image_height).into_iter().rev() {
        let v = j as Float / (image_height - 1) as Float;
        for i in 0..image_width {
            let u = i as Float / (image_width - 1) as Float;
            let mut color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
                let ray = cam.get_ray(
                    u + random::<Float>() / (image_width - 1) as Float,
                    v + random::<Float>() / (image_height - 1) as Float,
                );
                color += ray_color(&ray, &background, world, max_depth);
            }
            image.push(color);
        }
    }

    println!("finish render");
    return image;
}
