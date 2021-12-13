extern crate image;
extern crate rand;
extern crate raytracing;

use image::{ImageBuffer, ImageFormat, RgbImage};
use rand::Rng;

use raytracing::camera::Camera;
use raytracing::hit::{Hit, World};
use raytracing::material::Lambertian;
use raytracing::ray::Ray;
use raytracing::sphere::Sphere;
use raytracing::vec::{Color, Point3, Vec3};

fn ray_color(r: &Ray, world: &World, depth: u64) -> Color {
    if depth == 0 {
        // if we've exceded the allowed number of ray bounces, stop gathering more info
        return Color::new(0.0, 0.0, 0.0);
    }
    if let Some(rec) = world.hit(r, 0.001, f64::INFINITY) {
        // let target = rec.p + rec.normal + Vec3::random_in_unit_sphere();
        // let target = rec.p + rec.normal + Vec3::random_in_unit_sphere().normalized();
        if let Some((attenuation, scattered)) = rec.mat.scatter(r, &rec) {
            attenuation * ray_color(&scattered, world, depth - 1)
        } else {
            Color::new(0.0, 0.0, 0.0)
        }
    } else {
        let unit_direction = r.direction().normalized();
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }
}

fn main() {
    // image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u64 = 400;
    const IMAGE_HEIGHT: u64 = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as u64;
    const SAMPLES_PER_PIXEL: u64 = 100;
    const MAX_DEPTH: u64 = 5;

    // world
    let mut world = World::new();
    let red_matte = Lambertian::new(Color::new(0.3, 0.0, 0.0));
    let sphere = Sphere::new(Point3::new(0.0, 0.0, -1.0), -0.5, &red_matte);
    let ground = Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, &red_matte);
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
                pixel_color += ray_color(&r, &world, MAX_DEPTH);
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
