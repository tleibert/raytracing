//! Defines an abstraction for an object
//! that can be hit by a ray. Anything that can
//! be drawn needs to implement this trait.

use super::ray::Ray;
use super::vec::{Point3, Vec3};

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
}

pub trait Hit {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
