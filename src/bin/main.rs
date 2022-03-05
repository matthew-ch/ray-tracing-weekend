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

fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    let ground_material = Lambertian::new(Color::new(0.5, 0.5, 0.5));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Some(Box::new(ground_material)),
    )));

    for a in -11..11 {
        for b in -11..11 {
            let center = Point3::new(
                a as Float + 0.9 * random::<Float>(),
                0.2,
                b as Float + 0.9 * random::<Float>(),
            );
            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let choose = random::<Float>();
                let material: Box<dyn Material> = match choose {
                    n if n < 0.8 => Box::new(Lambertian::new(Color::random() * Color::random())),
                    n if n < 0.95 => Box::new(Metal::new(
                        Color::random_range(0.5..1.0),
                        random::<Float>() * 0.5,
                    )),
                    _ => Box::new(Dielectric::new(1.5)),
                };
                world.add(if choose < 0.8 {
                    let center1 = center + Vec3::new(0.0, random::<Float>() * 0.5, 0.0);
                    Box::new(MovingSphere::new(
                        center,
                        center1,
                        0.0,
                        1.0,
                        0.2,
                        Some(material),
                    ))
                } else {
                    Box::new(Sphere::new(center, 0.2, Some(material)))
                });
            }
        }
    }

    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        Some(Box::new(Dielectric::new(1.5))),
    )));

    world.add(Box::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        Some(Box::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)))),
    )));

    world.add(Box::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        Some(Box::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0))),
    )));

    world
}

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as Float / aspect_ratio) as u32;
    let samples_per_pixel = 10;
    let max_depth = 50;

    let world = random_scene();

    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        aspect_ratio,
        0.1,
        dist_to_focus,
        0.0,
        1.0,
    );

    let world_ref = unsafe { transmute::<_, &'static HittableList>(&world) };

    let n_threads = 10;

    let threads: Vec<_> = (0..n_threads)
        .map(|_| {
            spawn(move || {
                render(
                    world_ref,
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
