mod camera;
mod materials;
mod math_utils;
mod ray;
mod vec3;

use std::{f64::INFINITY, sync::Arc, u64};

use camera::*;
use materials::*;
use math_utils::*;
use ray::*;
use vec3::*;

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const WIDTH: u64 = 400;
const HEIGHT: u64 = (WIDTH as f64 / ASPECT_RATIO) as u64;
const SAMPLES_PER_PIXEL: u64 = 25;
const MAX_DEPTH: u64 = 10;

fn ray_colour(world: &Vec<Arc<dyn Hittable>>, r: &Ray, depth: u64) -> Colour {
    if depth <= 0 {
        return Colour::new(0.0, 0.0, 0.0);
    }

    match hit_world(world, r, 0.001, INFINITY) {
        Some(record) => match record.material.scatter(r, &record) {
            Some((scattered, attenuation)) => {
                attenuation * ray_colour(world, &scattered, depth - 1)
            }
            None => Colour::new(0.0, 0.0, 0.0),
        },
        None => {
            let unit_direction = r.dir.normalized();
            let t = 0.5 * (unit_direction.y + 1.0);
            lerp(t, &Colour::new(1.0, 1.0, 1.0), &Colour::new(0.5, 0.7, 1.0))
        }
    }
}

fn hit_world(
    world: &Vec<Arc<dyn Hittable>>,
    ray: &Ray,
    t_min: f64,
    t_max: f64,
) -> Option<HitRecord> {
    let mut closest = INFINITY;
    let mut hit_idx = 0;
    let mut hit = false;

    for i in 0..world.len() {
        match world[i].hit(ray, t_min, t_max) {
            Some(record) => {
                hit = true;
                if record.t < closest {
                    hit_idx = i;
                    closest = record.t;
                }
            }
            None => {}
        }
    }

    if hit {
        world[hit_idx].hit(ray, t_min, t_max)
    } else {
        None
    }
}

fn main() {
    let camera = Camera::new();

    let mut world: Vec<Arc<dyn Hittable>> = Vec::new();

    let material_ground = Arc::new(Diffuse {
        albedo: Colour::new(0.8, 0.8, 0.0),
    });
    let material_center = Arc::new(Diffuse {
        albedo: Colour::new(0.7, 0.3, 0.3),
    });
    let material_left = Arc::new(Dielectric {
        refraction_index: 1.5,
    });
    let material_right = Arc::new(Metal {
        albedo: Colour::new(0.8, 0.6, 0.2),
        fuzz: 0.1,
    });

    world.push(Arc::new(Sphere {
        centre: Point3::new(0.0, -100.5, -1.0),
        radius: 100.0,
        material: material_ground.clone(),
    }));
    world.push(Arc::new(Sphere {
        centre: Point3::new(0.0, 0.0, -1.0),
        radius: 0.5,
        material: material_center.clone(),
    }));
    world.push(Arc::new(Sphere {
        centre: Point3::new(-1.0, 0.0, -1.0),
        radius: 0.5,
        material: material_left.clone(),
    }));
    world.push(Arc::new(Sphere {
        centre: Point3::new(1.0, 0.0, -1.0),
        radius: 0.5,
        material: material_right.clone(),
    }));

    println!("P3 {} {} 255", WIDTH, HEIGHT);
    for j in 0..HEIGHT {
        eprintln!("Scanlines remaining: {}", HEIGHT - j);
        for i in 0..WIDTH {
            let mut colour = Colour::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + rand::random::<f64>()) / (WIDTH - 1) as f64;
                let v = ((HEIGHT - j - 1) as f64 + rand::random::<f64>()) / (HEIGHT - 1) as f64;
                let ray = camera.get_ray(u, v);
                colour += ray_colour(&world, &ray, MAX_DEPTH);
            }

            colour /= SAMPLES_PER_PIXEL as f64;
            colour.x = f64::sqrt(colour.x);
            colour.y = f64::sqrt(colour.y);
            colour.z = f64::sqrt(colour.z);

            println!("{}", colour_string(&colour));
        }
    }
    eprintln!("Done!");
}
