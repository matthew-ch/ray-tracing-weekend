mod aabb;
mod block_box;
mod bvh_node;
mod camera;
mod constant_medium;
mod flip_face;
mod hittable;
mod hittable_list;
mod material;
mod moving_sphere;
mod onb;
mod pdf;
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
use std::ops::Range;

pub use aabb::AABB;
pub use block_box::BlockBox;
pub use bvh_node::BvhNode;
pub use camera::Camera;
pub use constant_medium::ConstantMedium;
pub use flip_face::FlipFace;
pub use hittable::*;
pub use hittable_list::HittableList;
pub use material::*;
pub use moving_sphere::MovingSphere;
pub use onb::Onb;
pub use pdf::*;
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

pub fn random_range(range: Range<Float>) -> Float {
    range.start + random::<Float>() * (range.end - range.start)
}

pub fn ray_color<'a>(
    ray: &Ray,
    background: &Color,
    world: &'a impl Hittable,
    lights: &'a dyn Hittable,
    depth: i32,
) -> Color {
    if depth <= 0 {
        return Color::default();
    }
    let mut rec = HitRecord::default();
    if !world.hit(ray, 0.001, f64::INFINITY, &mut rec) {
        return background.clone();
    }
    let mut srec = ScatterRecord::default();
    let emitted = rec.material.map_or(Color::default(), |m| {
        m.emitted(ray, &rec, rec.u, rec.v, &rec.p)
    });

    if !rec
        .material
        .map_or(false, |mat| mat.scatter(ray, &rec, &mut srec))
    {
        emitted
    } else if let Some(specular_ray) = srec.specular_ray {
        srec.attenuation * ray_color(&specular_ray, background, world, lights, depth - 1)
    } else {
        let light = HittablePdf::new(lights, rec.p);
        let mixed_pdf = MixturePdf::new(&light, srec.pdf_ptr.as_deref().unwrap());

        let scattered = Ray::new(rec.p, mixed_pdf.generate(), ray.time());
        let pdf_val = mixed_pdf.value(&scattered.direction());

        emitted
            + srec.attenuation
                * rec.material.unwrap().scattering_pdf(ray, &rec, &scattered)
                * ray_color(&scattered, background, world, lights, depth - 1)
                / pdf_val
    }
}

pub fn render<'a>(
    world: &'a impl Hittable,
    lights: &'a dyn Hittable,
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
                color += ray_color(&ray, &background, world, lights, max_depth);
            }
            image.push(color);
        }
    }

    println!("finish render");
    return image;
}
