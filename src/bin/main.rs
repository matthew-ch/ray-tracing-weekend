use std::{
    fs::File,
    io::{BufWriter, Write},
    mem::size_of,
    path::Path,
};

use ray_tracing_weekend::{Color, Point3, Ray, Vec3, Float, hit_sphere};

fn write_image_png(data: &[u8], width: u32, height: u32, w: impl Write) {
    let mut encoder = png::Encoder::new(w, width, height);
    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(data).unwrap();
}

fn ray_color(ray: &Ray) -> Color {
    if let Some(t) = hit_sphere(&Point3::new(0.0, 0.0, -1.0), 0.5, ray) {
        let n = ray.at(t) - Point3::new(0.0, 0.0, -1.0);
        let n = n.unit_vector();
        0.5 * Color::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0)
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

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    let mut data: Vec<[u8; 3]> = Vec::with_capacity((image_width * image_height) as usize);

    for j in (0..image_height).into_iter().rev() {
        let v = j as Float / (image_height - 1) as Float;
        for i in 0..image_width {
            let u = i as Float / (image_width - 1) as Float;
            let ray = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );
            let color = ray_color(&ray);
            data.push(color.into());
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
