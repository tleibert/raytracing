//! Provides a 3-dimensional vector struct
//! for use representing positions and vectors.
//! It serves triple duty, also representing colors.

use std::ops::Range;

use glam::Vec3A;
use image::Rgb;
use rand::Rng;

/// Provides useful functions for a graphics-type vector
pub trait Graphics {
    /// produces a random vector with x,y,z pulled from the given range
    fn random(r: Range<f32>) -> Self;
    /// produces a random vector inside the unit sphere
    fn random_in_unit_sphere() -> Self;
    /// produces a random vector inside the unit hemisphere containing the normal
    fn random_in_hemisphere(normal: Self) -> Self;
    /// produces a random vector inside the unit disk
    fn random_in_unit_disk() -> Self;
    /// returns true if the vector is close to zero in all axes
    fn near_zero(self) -> bool;
    /// reflects the vector according to snell's law
    fn reflect(self, n: Self) -> Self;
    /// refracts the vector according to snell's law
    fn refract(self, n: Self, etai_over_etat: f32) -> Self;
    /// converts the vector to an rgb pixel value
    fn to_rgb(self, samples_per_pixel: u64) -> Rgb<u8>;
}

impl Graphics for Vec3A {
    #[inline(always)]
    fn random(r: Range<f32>) -> Self {
        let mut rng = rand::thread_rng();
        Self::new(
            rng.gen_range(r.clone()),
            rng.gen_range(r.clone()),
            rng.gen_range(r),
        )
    }

    #[inline(always)]
    fn random_in_unit_sphere() -> Self {
        loop {
            let v = Self::random(-1.0..1.0);
            if v.length() < 1.0 {
                return v;
            }
        }
    }

    #[inline(always)]
    fn random_in_hemisphere(normal: Self) -> Self {
        let in_unit_sphere = Self::random_in_unit_sphere();
        if in_unit_sphere.dot(normal) > 0.0 {
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }

    #[inline(always)]
    fn random_in_unit_disk() -> Self {
        let mut rng = rand::thread_rng();

        loop {
            let p = Self::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);
            if p.length() < 1.0 {
                return p;
            }
        }
    }

    #[inline(always)]
    fn near_zero(self) -> bool {
        const EPS: f32 = 1.0e-8;
        self[0].abs() < EPS && self[1].abs() < EPS && self[2].abs() < EPS
    }

    #[inline(always)]
    fn reflect(self, n: Self) -> Self {
        self - 2.0 * self.dot(n) * n
    }

    #[inline(always)]
    fn refract(self, n: Self, etai_over_etat: f32) -> Self {
        let cos_theta = (-self).dot(n).min(1.0);
        let r_out_perp = etai_over_etat * (self + cos_theta * n);
        let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * n;
        r_out_perp + r_out_parallel
    }

    #[inline(always)]
    fn to_rgb(self, samples_per_pixel: u64) -> Rgb<u8> {
        let ir = (256.0
            * (self[0] / (samples_per_pixel as f32))
                .sqrt()
                .clamp(0.0, 0.999)) as u8;
        let ig = (256.0
            * (self[1] / (samples_per_pixel as f32))
                .sqrt()
                .clamp(0.0, 0.999)) as u8;
        let ib = (256.0
            * (self[2] / (samples_per_pixel as f32))
                .sqrt()
                .clamp(0.0, 0.999)) as u8;

        let arr = [ir, ig, ib];
        Rgb::from(arr)
    }
}

pub type Vec3 = Vec3A;
pub type Point3 = Vec3;
pub type Color = Vec3;
