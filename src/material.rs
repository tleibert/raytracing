//! Provides a material trait

use super::hit::HitRecord;
use super::ray::Ray;
use super::vec::Color;
use crate::vec::Vec3;

pub trait Scatter: Send + Sync {
    /// Runs when an object is "hit", yielding a new, "bounced" ray and a color.
    /// Returns none if the ray isn't bouncing again.
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>;
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Lambertian { albedo }
    }
}

impl Scatter for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        // let scatter_direction = rec.p + rec.normal + Vec3::random_in_unit_sphere();
        let mut scatter_direction = rec.normal + Vec3::random_in_unit_sphere().normalized();
        // let target = rec.p + Vec3::random_in_hemisphere(rec.normal);

        // catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        let scattered = Ray::new(rec.p, scatter_direction);

        Some((self.albedo, scattered))
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Metal { albedo, fuzz }
    }
}

impl Scatter for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = r_in.direction().reflect(rec.normal).normalized();
        let scattered = Ray::new(rec.p, reflected + self.fuzz * Vec3::random_in_unit_sphere());

        if scattered.direction().dot(rec.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}

pub struct Dielectric {
    index_of_refraction: f64,
}

impl Dielectric {
    pub fn new(index_of_refraction: f64) -> Self {
        Self {
            index_of_refraction,
        }
    }
}

impl Scatter for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let refraction_ratio = if rec.front_face {
            1.0 / self.index_of_refraction
        } else {
            self.index_of_refraction
        };

        let unit_direction = r_in.direction().normalized();
        let refracted = unit_direction.refract(rec.normal, refraction_ratio);
        let scattered = Ray::new(rec.p, refracted);

        Some((Color::new(1.0, 1.0, 1.0), scattered))
    }
}
