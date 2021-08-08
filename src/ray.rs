use std::sync::Arc;

use crate::{materials::*, vec3::*};

#[derive(Clone, Copy)]
pub struct Ray {
    pub origin: Point3,
    pub dir: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, dir: Vec3) -> Ray {
        Ray { origin, dir }
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + self.dir * t
    }
}

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub material: Arc<dyn Material>,
}

impl HitRecord {
    pub fn new(normal: Vec3, t: f64, material: Arc<dyn Material>, ray: &Ray) -> HitRecord {
        let mut ans = HitRecord {
            p: ray.at(t),
            normal,
            t,
            front_face: false,
            material,
        };
        ans.set_normal(ray, &normal);
        ans
    }

    fn set_normal(&mut self, ray: &Ray, normal: &Vec3) {
        self.front_face = Vec3::dot(&ray.dir, normal) < 0.0;
        self.normal = if self.front_face { *normal } else { -*normal };
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct Sphere {
    pub centre: Point3,
    pub radius: f64,
    pub material: Arc<dyn Material>,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let origin = ray.origin - self.centre;
        let a = ray.dir.len_squared();
        let half_b = Vec3::dot(&origin, &ray.dir);
        let c = origin.len_squared() - self.radius.powi(2);

        let discriminant = half_b.powi(2) - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrt_d = f64::sqrt(discriminant);

        let mut root = (-half_b - sqrt_d) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrt_d) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let normal = (ray.at(root) - self.centre) / self.radius;
        Some(HitRecord::new(normal, root, self.material.clone(), &ray))
    }
}
