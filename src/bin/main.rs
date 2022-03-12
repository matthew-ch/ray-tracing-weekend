use std::{
    fs::File,
    intrinsics::transmute,
    io::{BufWriter, Read, Write},
    mem::size_of,
    path::Path,
    sync::Arc,
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

fn read_png(file: impl Read) -> (Vec<u8>, u32, u32) {
    let mut decoder = png::Decoder::new(file);
    decoder.set_transformations(png::Transformations::EXPAND);
    let mut reader = decoder.read_info().unwrap();
    let mut buf = vec![0; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buf).unwrap();
    buf.truncate(info.buffer_size());
    (buf, info.width, info.height)
}

fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    let checker =
        CheckerTexture::new_with_color(Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9));
    let ground_material = Lambertian::new_with_texture(checker);
    world.add(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(ground_material),
    ));

    for a in -11..11 {
        for b in -11..11 {
            let center = Point3::new(
                a as Float + 0.9 * random::<Float>(),
                0.2,
                b as Float + 0.9 * random::<Float>(),
            );
            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let choose = random::<Float>();
                if choose < 0.8 {
                    let center1 = center + Vec3::new(0.0, random::<Float>() * 0.5, 0.0);
                    world.add(MovingSphere::new(
                        center,
                        center1,
                        0.0,
                        1.0,
                        0.2,
                        Arc::new(Lambertian::new_with_color(
                            Color::random() * Color::random(),
                        )),
                    ));
                } else if choose < 0.95 {
                    world.add(Sphere::new(
                        center,
                        0.2,
                        Arc::new(Metal::new(
                            Color::random_range(0.5..1.0),
                            random::<Float>() * 0.5,
                        )),
                    ));
                } else {
                    world.add(Sphere::new(center, 0.2, Arc::new(Dielectric::new(1.5))));
                }
            }
        }
    }

    world.add(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        Arc::new(Dielectric::new(1.5)),
    ));

    world.add(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        Arc::new(Lambertian::new_with_color(Color::new(0.4, 0.2, 0.1))),
    ));

    world.add(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0)),
    ));

    world
}

fn two_spheres() -> HittableList {
    let mut objects = HittableList::new();

    for y in [-10.0, 10.0] {
        let checker =
            CheckerTexture::new_with_color(Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9));

        objects.add(Sphere::new(
            Point3::new(0.0, y, 0.0),
            10.0,
            Arc::new(Lambertian::new_with_texture(checker)),
        ));
    }

    objects
}

fn two_perlin_spheres() -> HittableList {
    let mut objects = HittableList::new();

    for (p, r) in [
        (Point3::new(0.0, -1000.0, 0.0), 1000.0),
        (Point3::new(0.0, 2.0, 0.0), 2.0),
    ] {
        objects.add(Sphere::new(
            p,
            r,
            Arc::new(Lambertian::new_with_texture(NoiseTexture::new(4.0))),
        ));
    }

    objects
}

fn earth() -> HittableList {
    let (data, width, height) = read_png(File::open(Path::new(r"./earthmap.png")).unwrap());
    let earth_texture = ImageTexture::new(data, width, height);
    let earth_surface = Lambertian::new_with_texture(earth_texture);
    let globe = Sphere::new(Point3::new(0.0, 0.0, 0.0), 2.0, Arc::new(earth_surface));
    HittableList::new_with(globe)
}

fn simple_light() -> HittableList {
    let mut objects = two_perlin_spheres();

    let light = Arc::new(DiffuseLight::new_with_color(Color::new(4.0, 4.0, 4.0)));

    objects.add(Sphere::new(Point3::new(0.0, 7.0, 0.0), 2.0, light.clone()));

    objects.add(XyRect::new(3.0, 5.0, 1.0, 3.0, -2.0, light));

    objects
}

fn cornell_box() -> HittableList {
    let mut objects = HittableList::new();

    let red = Arc::new(Lambertian::new_with_color(Color::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new_with_color(Color::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new_with_color(Color::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::new_with_color(Color::new(15.0, 15.0, 15.0)));

    objects.add(YzRect::new(0.0, 555.0, 0.0, 555.0, 555.0, green));

    objects.add(YzRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red));

    objects.add(XzRect::new(213.0, 343.0, 227.0, 332.0, 554.0, light));

    objects.add(XzRect::new(0.0, 555.0, 0.0, 555.0, 0.0, white.clone()));

    objects.add(XzRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white.clone()));

    objects.add(XyRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white.clone()));

    let object = BlockBox::new(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 330.0, 165.0),
        white.clone(),
    );
    let object = RotateY::new(Arc::new(object), 15.0);
    let object = Translate::new(Arc::new(object), Vec3::new(265.0, 0.0, 295.0));
    objects.add(object);

    let object = BlockBox::new(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 165.0, 165.0),
        white,
    );
    let object = RotateY::new(Arc::new(object), -18.0);
    let object = Translate::new(Arc::new(object), Vec3::new(130.0, 0.0, 65.0));
    objects.add(object);

    objects
}

fn main() {
    let mut aspect_ratio = 16.0 / 9.0;
    let mut image_width = 400;
    let mut samples_per_pixel = 10;
    let max_depth = 50;

    let mut lookfrom = Point3::new(13.0, 2.0, 3.0);
    let mut lookat = Point3::new(0.0, 0.0, 0.0);
    let mut vfov = 20.0;
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let mut aperture = 0.1;
    let mut focus_dist = 10.0;
    let mut background = Color::new(0.7, 0.8, 1.0);

    let world = match 0 {
        1 => random_scene(),
        2 => {
            aperture = 0.0;
            two_spheres()
        }
        3 => two_perlin_spheres(),
        4 => earth(),
        5 => {
            background = Color::default();
            samples_per_pixel = 40;
            focus_dist = 20.0;
            lookfrom = Point3::new(26.0, 3.0, 6.0);
            lookat = Point3::new(0.0, 2.0, 0.0);
            simple_light()
        }
        _ => {
            aspect_ratio = 1.0;
            image_width = 600;
            samples_per_pixel = 20;
            lookfrom = Point3::new(278.0, 278.0, -800.0);
            lookat = Point3::new(278.0, 278.0, 0.0);
            background = Color::default();
            vfov = 40.0;
            aperture = 0.0;
            cornell_box()
        }
    };

    let image_height = (image_width as Float / aspect_ratio) as u32;

    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        vfov,
        aspect_ratio,
        aperture,
        focus_dist,
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
                    background,
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
