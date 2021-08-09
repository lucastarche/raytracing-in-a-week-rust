use crate::math_utils::random_in_unit_disk;
use crate::ray::*;
use crate::vec3::*;

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f64,
}

impl Camera {
    pub fn new(
        look_from: Point3,
        look_at: Point3,
        vec_up: Vec3,
        vert_fov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Camera {
        let viewport_height = 2.0 * f64::tan(vert_fov.to_radians() / 2.0);
        let viewport_width = aspect_ratio * viewport_height;

        let w = (look_from - look_at).normalized();
        let u = Vec3::cross(&vec_up, &w).normalized();
        let v = Vec3::cross(&w, &u);

        let origin = look_from;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - focus_dist * w;

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            w,
            lens_radius: aperture / 2.0,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let random_disk = self.lens_radius * random_in_unit_disk();
        let offset = self.u * random_disk.x + self.v * random_disk.y;
        Ray {
            origin: self.origin + offset,
            dir: self.lower_left_corner + u * self.horizontal + v * self.vertical
                - self.origin
                - offset,
        }
    }
}
