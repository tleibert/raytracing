extern crate image;
extern crate rand;
extern crate raytracing;

use image::{ImageBuffer, ImageFormat, RgbImage};
use rand::Rng;

use raytracing::camera::Camera;
use raytracing::hit::{Hit, World};
use raytracing::ray::Ray;
use raytracing::sphere::Sphere;
use raytracing::vec::{Color, Point3, Vec3};

fn ray_color(r: &Ray, world: &World) -> Color {
    if let Some(rec) = world.hit(r, 0.0, f64::INFINITY) {
        0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0))
    } else {
        let unit_direction = r.direction().normalized();
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }
}

fn main() {
    // image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u64 = 1920;
    const IMAGE_HEIGHT: u64 = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as u64;
    const SAMPLES_PER_PIXEL: u64 = 100;

    // world
    let mut world = World::new();
    let sphere = Sphere::new(Point3::new(0.0, 0.0, -1.0), -0.5);
    let ground = Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0);
    world.push(&sphere);
    world.push(&ground);

    // camera
    let cam = Camera::new();
    // println!("P3");
    // println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    // println!("255");

    let mut rng = rand::thread_rng();
    let mut image_buffer: RgbImage = ImageBuffer::new(IMAGE_WIDTH as u32, IMAGE_HEIGHT as u32);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("Scanlines remaining: {}", j);
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXEL {
                let random_u: f64 = rng.gen();
                let random_v: f64 = rng.gen();

                let u = ((i as f64) + random_u) / ((IMAGE_WIDTH - 1) as f64);
                let v = ((j as f64) + random_v) / ((IMAGE_HEIGHT - 1) as f64);

                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world);
            }

            let pixel = pixel_color.to_rgb(SAMPLES_PER_PIXEL);
            image_buffer.put_pixel(i as u32, (IMAGE_HEIGHT - j - 1) as u32, pixel);

            // println!("{}", pixel_color.format_color(SAMPLES_PER_PIXEL));
        }
    }

    image_buffer
        .save_with_format("image.png", ImageFormat::Png)
        .unwrap();

    eprintln!("Done.");
}
