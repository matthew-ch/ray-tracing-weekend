use std::{
    fs::File,
    io::{BufWriter, Write},
    mem::size_of,
    path::Path,
};

use rand::random;
use ray_tracing_weekend::{
    make_shared_material, Camera, Color, Dielectric, Float, HitRecord, Hittable, HittableList,
    Lambertian, Metal, Point3, Ray, Sphere,
};

fn write_image_png(data: &[u8], width: u32, height: u32, w: impl Write) {
    let mut encoder = png::Encoder::new(w, width, height);
    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(data).unwrap();
}

fn ray_color(ray: &Ray, world: &impl Hittable, depth: i32) -> Color {
    if depth <= 0 {
        return Color::default();
    }
    let mut rec = HitRecord::default();
    if world.hit(ray, 0.001, f64::INFINITY, &mut rec) {
        let mut scattered = Ray::default();
        let mut attenuation = Color::default();
        if rec.material.clone().map_or(false, |mat| {
            mat.scatter(ray, &mut rec, &mut attenuation, &mut scattered)
        }) {
            attenuation * ray_color(&scattered, world, depth - 1)
        } else {
            Color::default()
        }
    } else {
        let dir = ray.direction().unit_vector();
        let t = 0.5 * (dir.y() + 1.0);
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }
}

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as Float / aspect_ratio) as u32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    let mut world = HittableList::new();
    let material_ground = make_shared_material(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = make_shared_material(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = make_shared_material(Dielectric::new(1.5));
    let material_right = make_shared_material(Metal::new(Color::new(0.8, 0.6, 0.2), 0.0));

    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        Some(material_ground),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        Some(material_center),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        Some(material_left.clone()),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        -0.4,
        Some(material_left),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        Some(material_right),
    )));

    let cam = Camera::new();

    let mut data: Vec<[u8; 3]> = Vec::with_capacity((image_width * image_height) as usize);

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
                color += ray_color(&ray, &world, max_depth);
            }
            color /= samples_per_pixel as Float;
            data.push(color.apply(Float::sqrt).into());
        }
    }

    let path = Path::new(r"./output.png");
    let file = File::create(path).unwrap();
    let w = BufWriter::new(file);

    let data = unsafe {
        let ptr = data.as_ptr();
        std::slice::from_raw_parts(ptr as *const u8, data.len() * size_of::<[u8; 3]>())
    };
    write_image_png(data, image_width, image_height, w);
}
