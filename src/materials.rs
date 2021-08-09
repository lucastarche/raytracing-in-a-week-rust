use crate::{math_utils::*, ray::*, vec3::*};

pub trait Material {
    fn scatter(&self, ray_in: &Ray, record: &HitRecord) -> Option<(Ray, Colour)>;
}

pub struct Diffuse {
    pub albedo: Colour,
}

pub struct Metal {
    pub albedo: Colour,
    pub fuzz: f64,
}

pub struct Dielectric {
    pub refraction_index: f64,
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
        let scattered = Ray::new(record.p, reflected + self.fuzz * random_in_unit_sphere());

        if Vec3::dot(&reflected, &scattered.dir) > 0.0 {
            Some((scattered, self.albedo))
        } else {
            None
        }
    }
}

impl Dielectric {
    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        // Use Schlick's approximation for reflectance.
        let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);
        r0 + (1.0 - r0) * f64::powi(1.0 - cosine, 5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, record: &HitRecord) -> Option<(Ray, Colour)> {
        let refraction_ratio = if record.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_dir = ray_in.dir.normalized();
        let cos_theta = f64::min(Vec3::dot(&-unit_dir, &record.normal), 1.0);
        let sin_theta = f64::sqrt(1.0 - cos_theta * cos_theta);

        let direction = if refraction_ratio * sin_theta > 1.0
            || Dielectric::reflectance(cos_theta, self.refraction_index) > rand::random()
        {
            unit_dir.reflect(&record.normal)
        } else {
            unit_dir.refract(&record.normal, refraction_ratio)
        };

        Some((
            Ray {
                origin: record.p,
                dir: direction,
            },
            Colour::new(1.0, 1.0, 1.0),
        ))
    }
}
