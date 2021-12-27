extern crate image;
extern crate rand;
extern crate raytracing;

use std::sync::Arc;

use image::{ImageBuffer, ImageFormat, RgbImage};
use rand::Rng;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use raytracing::camera::Camera;
use raytracing::hit::{Hit, World};
use raytracing::material::{Dielectric, Lambertian, Metal};
use raytracing::ray::Ray;
use raytracing::sphere::Sphere;
use raytracing::vec::{Color, Point3, Vec3};

fn ray_color(r: &Ray, world: &World, depth: u64) -> Color {
    if depth == 0 {
        // if we've exceeded the allowed number of ray bounces, stop gathering more info
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
    const IMAGE_WIDTH: u64 = 1920;
    const IMAGE_HEIGHT: u64 = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as u64;
    const SAMPLES_PER_PIXEL: u64 = 100;
    const MAX_DEPTH: u64 = 5;

    // world
    let mut world = World::new();
    let mat_ground = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let mat_center = Arc::new(Dielectric::new(1.5));
    let mat_left = Arc::new(Dielectric::new(1.5));
    let mat_right = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));

    let sphere_ground = Arc::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        mat_ground,
    ));
    let sphere_center = Arc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, mat_center));
    let sphere_left = Arc::new(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, mat_left));
    let sphere_right = Arc::new(Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, mat_right));

    world.push(sphere_ground);
    world.push(sphere_center);
    world.push(sphere_left);
    world.push(sphere_right);

    // camera
    let cam = Camera::new();
    // println!("P3");
    // println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    // println!("255");

    let mut image_buffer: RgbImage = ImageBuffer::new(IMAGE_WIDTH as u32, IMAGE_HEIGHT as u32);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("Scanlines remaining: {}", j);
        let scanline: Vec<Color> = (0..IMAGE_WIDTH)
            .into_par_iter()
            .map(|i| {
                let mut rng = rand::thread_rng();
                (0..SAMPLES_PER_PIXEL)
                    .into_iter()
                    .map(|_| {
                        let random_u: f64 = rng.gen();
                        let random_v: f64 = rng.gen();

                        let u = ((i as f64) + random_u) / ((IMAGE_WIDTH - 1) as f64);
                        let v = ((j as f64) + random_v) / ((IMAGE_HEIGHT - 1) as f64);

                        let r = cam.get_ray(u, v);
                        ray_color(&r, &world, MAX_DEPTH)
                    })
                    .sum()

                // println!("{}", pixel_color.format_color(SAMPLES_PER_PIXEL));
            })
            .collect();

        for (i, pixel) in scanline.into_iter().enumerate() {
            let pixel = pixel.to_rgb(SAMPLES_PER_PIXEL);
            image_buffer.put_pixel(i as u32, (IMAGE_HEIGHT - j - 1) as u32, pixel);
        }
    }

    image_buffer
        .save_with_format("image.png", ImageFormat::Png)
        .unwrap();

    eprintln!("Done.");
}
