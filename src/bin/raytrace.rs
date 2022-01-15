extern crate image;
extern crate rand;
extern crate raytracing;

use std::sync::Arc;

use image::{ImageBuffer, ImageFormat, RgbImage};
use rand::Rng;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use raytracing::camera::Camera;
use raytracing::hit::{Hit, World};
use raytracing::material::{Dielectric, Lambertian, Metal, Scatter};
use raytracing::ray::Ray;
use raytracing::sphere::Sphere;
use raytracing::vec::{Color, Point3};

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

fn random_scene() -> World {
    let mut world = World::new();

    let ground_material = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    let ground = Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    );

    world.push(ground);

    let glass_mat = Arc::new(Dielectric::new(1.5));

    let mut rng = rand::thread_rng();
    for a in -11..11 {
        for b in -11..11 {
            let center = Point3::new(
                (a as f64) + rng.gen_range(0.0..0.9),
                0.2,
                (b as f64) + rng.gen_range(0.0..0.9),
            );

            // make sure spheres aren't inside the big center spheres
            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let choose_mat: f64 = rng.gen();

                let mat: Arc<dyn Scatter> = if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random(0.0..1.0) * Color::random(0.0..1.0);
                    Arc::new(Lambertian::new(albedo))
                } else if choose_mat < 0.95 {
                    let albedo = Color::random(0.4..1.0);
                    let fuzz = rng.gen_range(0.0..0.5);
                    Arc::new(Metal::new(albedo, fuzz))
                } else {
                    glass_mat.clone()
                };
                let sphere = Sphere::new(center, 0.2, mat);

                world.push(sphere);
            }
        }
    }

    // 3 main spheres
    let diffuse = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    let metal = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));

    let big_glass = Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, glass_mat.clone());
    let big_diffuse = Sphere::new(Point3::new(-8.0, 1.0, 0.0), 1.0, diffuse);
    let big_metal = Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, metal);
    let big_hollow = Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, glass_mat.clone());
    let big_hollow_inner = Sphere::new(Point3::new(4.0, 1.0, 0.0), -0.8, glass_mat);

    world.push(big_glass);
    world.push(big_diffuse);
    world.push(big_metal);
    world.push(big_hollow);
    world.push(big_hollow_inner);

    world
}

fn main() {
    // image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u64 = 1920;
    const IMAGE_HEIGHT: u64 = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as u64;
    const SAMPLES_PER_PIXEL: u64 = 50;
    const MAX_DEPTH: u64 = 10;

    // world
    let world = random_scene();

    // camera
    let origin = Point3::new(13.0, 2.0, 3.0);
    let look_at = Point3::new(0.0, 0.0, 0.0);
    let roll = 0.0;
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let cam = Camera::new(
        origin,
        look_at,
        roll,
        20.0,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
    );

    let mut image_buffer: RgbImage = ImageBuffer::new(IMAGE_WIDTH as u32, IMAGE_HEIGHT as u32);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("Scanlines remaining: {j}");
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
