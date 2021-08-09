mod camera;
mod materials;
mod math_utils;
mod ray;
mod vec3;

use std::{f64::INFINITY, fs::File, io::Write, sync::Arc, u64};

use camera::*;
use materials::*;
use math_utils::*;
use rand::{thread_rng, Rng};
use ray::*;
use vec3::*;

const ASPECT_RATIO: f64 = 3.0 / 2.0;
const WIDTH: u64 = 1200;
const HEIGHT: u64 = (WIDTH as f64 / ASPECT_RATIO) as u64;
const SAMPLES_PER_PIXEL: u64 = 500;
const MAX_DEPTH: u64 = 50;

fn random_world() -> Vec<Arc<dyn Hittable>> {
    let mut world: Vec<Arc<dyn Hittable>> = Vec::new();

    let ground_material = Arc::new(Diffuse {
        albedo: Colour::new(0.5, 0.5, 0.5),
    });

    world.push(Arc::new(Sphere {
        centre: Point3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: ground_material,
    }));

    for a in -11..11 {
        for b in -11..11 {
            let centre = Point3::new(
                a as f64 + 0.9 * rand::random::<f64>(),
                0.2,
                b as f64 + 0.9 * rand::random::<f64>(),
            );

            if (centre - Point3::new(4.0, 0.2, 0.0)).len() > 0.9 {
                let material: Arc<dyn Material> = match rand::random() {
                    x if (0.0..0.8).contains(&x) => Arc::new(Diffuse {
                        albedo: Colour::random() * Colour::random(),
                    }),
                    x if (0.8..0.95).contains(&x) => Arc::new(Metal {
                        albedo: Colour::random_range(0.5, 1.0),
                        fuzz: thread_rng().gen_range(0.0..0.5),
                    }),
                    _ => Arc::new(Dielectric {
                        refraction_index: 1.5,
                    }),
                };

                world.push(Arc::new(Sphere {
                    centre,
                    radius: 0.2,
                    material,
                }));
            }
        }
    }

    world.push(Arc::new(Sphere {
        centre: Point3::new(0.0, 1.0, 0.0),
        radius: 1.0,
        material: Arc::new(Dielectric {
            refraction_index: 1.5,
        }),
    }));

    world.push(Arc::new(Sphere {
        centre: Point3::new(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: Arc::new(Diffuse {
            albedo: Colour::new(0.4, 0.2, 0.1),
        }),
    }));

    world.push(Arc::new(Sphere {
        centre: Point3::new(4.0, 1.0, 0.0),
        radius: 1.0,
        material: Arc::new(Metal {
            albedo: Colour::new(0.7, 0.6, 0.5),
            fuzz: 0.0,
        }),
    }));

    world
}

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
    let mut file = File::create("a.ppm").expect("Couldnt open file");

    let look_from = Point3::new(13.0, 2.0, 3.0);
    let look_at = Point3::new(0.0, 0.0, 0.0);
    let vec_up = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let camera = Camera::new(
        look_from,
        look_at,
        vec_up,
        20.0,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
    );

    let world = random_world();

    file.write(format!("P3 {} {} 255\n", WIDTH, HEIGHT).as_bytes())
        .expect("Error writing to file");
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

            file.write(format!("{}\n", colour_string(&colour)).as_bytes())
                .expect("Error writing to file");
        }
    }
    eprintln!("Done!");
}
