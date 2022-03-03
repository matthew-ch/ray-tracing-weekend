use std::{
    fs::File,
    intrinsics::transmute,
    io::{BufWriter, Write},
    mem::size_of,
    path::Path,
    thread::spawn,
};

use rand::random;
use ray_tracing_weekend::*;

fn write_image_png(data: &[u8], width: u32, height: u32, w: impl Write) {
    let mut encoder = png::Encoder::new(w, width, height);
    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(data).unwrap();
}

fn ray_color<'a>(ray: &Ray, world: &'a impl Hittable, depth: i32) -> Color {
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

fn render<'a>(
    world: &'a impl Hittable,
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
                color += ray_color(&ray, world, max_depth);
            }
            image.push(color);
        }
    }

    println!("finish render");
    return image;
}

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as Float / aspect_ratio) as u32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    let material_ground = Lambertian::new(Color::new(0.8, 0.8, 0.0));
    let material_center = Lambertian::new(Color::new(0.1, 0.2, 0.5));
    let material_left = Dielectric::new(1.5);
    let material_right = Metal::new(Color::new(0.8, 0.6, 0.2), 0.0);
    let mut world = HittableList::new();

    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        Some(Box::new(material_ground)),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        Some(Box::new(material_center)),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        Some(Box::new(material_left)),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        -0.45,
        Some(Box::new(material_left)),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        Some(Box::new(material_right)),
    )));

    let lookfrom = Point3::new(3.0, 3.0, 2.0);
    let lookat = Point3::new(0.0, 0.0, -1.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = (lookfrom - lookat).length();
    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        aspect_ratio,
        2.0,
        dist_to_focus,
    );

    let world = unsafe { transmute::<_, &'static HittableList>(&world) };

    let n_threads = 10;

    let threads: Vec<_> = (0..n_threads)
        .map(|_| {
            spawn(move || {
                render(
                    world,
                    cam,
                    image_width,
                    image_height,
                    samples_per_pixel,
                    max_depth,
                )
            })
        })
        .collect();

    let images = threads.into_iter().map(|th| th.join().unwrap());

    let image = images
        .reduce(|mut accum, item| {
            for (i, color) in item.into_iter().enumerate() {
                accum[i] += color;
            }
            accum
        })
        .unwrap();

    let data: Vec<[u8; 3]> = image
        .into_iter()
        .map(|color| {
            (color / (samples_per_pixel * n_threads) as Float)
                .apply(Float::sqrt)
                .into()
        })
        .collect();

    let path = Path::new(r"./output.png");
    let file = File::create(path).unwrap();
    let w = BufWriter::new(file);

    let data = unsafe {
        let ptr = data.as_ptr();
        std::slice::from_raw_parts(ptr as *const u8, data.len() * size_of::<[u8; 3]>())
    };
    write_image_png(data, image_width, image_height, w);
}
