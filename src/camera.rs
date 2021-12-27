//! Provides an abstraction for a camera.

use crate::ray::Ray;

use super::vec::{Point3, Vec3};

#[derive(Debug)]
pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(origin: Point3, look_at: Point3, fov: f64, aspect_ratio: f64, roll: f64) -> Self {
        // convert to radians
        let roll_angle = roll.to_radians();
        let rotated_up = Vec3::new(-roll_angle.sin(), roll_angle.cos(), 0.0);
        let h = (fov.to_radians() / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (origin - look_at).normalized();
        let u = rotated_up.cross(w).normalized();
        let v = w.cross(u);

        let origin = origin;
        let horizontal = u * viewport_width;
        let vertical = v * viewport_height;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w;

        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + (s * self.horizontal) + (t * self.vertical) - self.origin,
        )
    }
}
