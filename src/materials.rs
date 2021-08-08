use crate::{math_utils::*, ray::*, vec3::*};

pub trait Material {
    fn scatter(&self, ray_in: &Ray, record: &HitRecord) -> Option<(Ray, Colour)>;
}

pub struct Diffuse {
    pub albedo: Colour,
}

pub struct Metal {
    pub albedo: Colour,
}

impl Material for Diffuse {
    fn scatter(&self, _ray_in: &Ray, record: &HitRecord) -> Option<(Ray, Colour)> {
        let scatter_direction = record.normal + random_unit_vector();

        if scatter_direction.near_zero() {
            Some((
                Ray {
                    origin: record.p,
                    dir: record.normal,
                },
                self.albedo,
            ))
        } else {
            Some((
                Ray {
                    origin: record.p,
                    dir: scatter_direction,
                },
                self.albedo,
            ))
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, record: &HitRecord) -> Option<(Ray, Colour)> {
        let reflected = ray_in.dir.reflect(&record.normal.normalized());

        if Vec3::dot(&reflected, &record.normal) > 0.0 {
            Some((Ray::new(record.p, reflected), self.albedo))
        } else {
            None
        }
    }
}
