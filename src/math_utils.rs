use std::ops::{Add, Mul};

use crate::vec3::Vec3;

pub fn lerp<T>(t: f64, start: &T, end: &T) -> <<T as Mul<f64>>::Output as Add>::Output
where
    T: Mul<f64> + Copy,
    T::Output: Add,
{
    *start * (1.0 - t) + *end * t
}

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = Vec3::random_range(-1.0, 1.0);
        if p.len_squared() < 1.0 {
            return p;
        }
    }
}

pub fn random_unit_vector() -> Vec3 {
    random_in_unit_sphere().normalized()
}

pub fn random_in_unit_disk() -> Vec3 {
    loop {
        let p = Vec3::new(rand::random(), rand::random(), 0.0);
        if p.len_squared() < 1.0 {
            return p;
        }
    }
}
