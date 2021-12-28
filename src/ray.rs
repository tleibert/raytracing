//! Provides an abstraction for a ray, which
//! has both a point of origin as well as a direction.
//!
//! Using these, the ray's position at any time `t`
//! can be calculated.

use super::vec::{Point3, Vec3};

pub struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Ray {
    pub fn new(orig: Point3, dir: Vec3) -> Ray {
        Ray { orig, dir }
    }

    pub fn origin(&self) -> Point3 {
        self.orig
    }

    pub fn direction(&self) -> Vec3 {
        self.dir
    }

    pub fn at(&self, t: f32) -> Point3 {
        self.orig + t * self.dir
    }
}
