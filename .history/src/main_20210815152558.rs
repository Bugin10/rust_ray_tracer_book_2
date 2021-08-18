use std::io::Error;
use std::time::Instant;

use indicatif::ProgressBar;
use rand::Rng;
use rayon::prelude::*;

// mod Vec3;
mod camera;
mod hittable;
mod hittable_list;
mod material;
mod moving_sphere;
mod ray;
mod sphere;
mod utility;
mod aabb;

// use Vec3::*;
use camera::*;
use hittable::*;
use hittable_list::*;
use material::*;
use moving_sphere::*;
use ray::*;
use sphere::*;
use utility::*;
use aabb::*;

use ultraviolet::*;

pub fn ray_color(r: &Ray, world: &dyn Hittable, depth: i32, max_depth: i32) -> Vec3 {
    match world.hit(r, 0.001, std::f32::INFINITY) {
        Some((hit_record, material)) => {
            let (scattered, attenuation, b) = material.scatter(r, &hit_record);
            if depth < max_depth && b {
                let res = attenuation * ray_color(&scattered, world, depth + 1, max_depth);
                res
            } else {
                Vec3::new(0.0, 0.0, 0.0)
            }
        }
        None => {
            let unit_direction = r.direction.normalized();
            let t = 0.5 * (unit_direction.y + 1.0);
            Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
        }
    }
}

pub fn main() -> Result<(), Error> {
    // Image config
    let aspect_ratio = 3.0 / 2.0;
    let image_width = 400;
    let image_height = (image_width as f32 / aspect_ratio) as i32;
    let num_samples = 100;
    let max_depth = 50;

    // world
    let mut world = HittableList::new();

    let ground_material = Material::Lambertian {
        albedo: Vec3::new(0.5, 0.5, 0.5),
    };

    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    let mut rng = rand::thread_rng();
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen::<f32>();
            let center = Vec3::new(
                a as f32 + 0.9 * rng.gen::<f32>(),
                0.2,
                b as f32 + 0.9 * rng.gen::<f32>(),
            );

            if (center - Vec3::new(4.0, 0.2, 0.0)).mag() > 0.9 {
                if choose_mat < 0.8 {
                    // Diffuse
                    let albedo = random_Vec3() * random_Vec3();
                    let sphere_material = Material::Lambertian { albedo };
                    let center2 = center + Vec3::new(0.0, rng.gen_range(0.0..0.5), 0.0);
                    world.add(Box::new(Moving_Sphere::new(
                        center,
                        center2,
                        0.0,
                        1.0,
                        0.2,
                        sphere_material,
                    )));
                } else if choose_mat < 0.95 {
                    // Metal
                    let albedo = random_Vec3() * random_Vec3();
                    let fuzz = rng.gen_range(0.0..0.5);
                    let sphere_material = Material::Metal {
                        albedo: albedo,
                        roughness: fuzz,
                    };
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    let sphere_material = Material::Dielectric {
                        index_refraction: 1.5,
                    };
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)))
                }
            }
        }
    }

    let material1 = Material::Dielectric {
        index_refraction: 1.5,
    };
    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Material::Lambertian {
        albedo: Vec3::new(0.4, 0.2, 0.1),
    };
    world.add(Box::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Material::Metal {
        albedo: Vec3::new(0.7, 0.6, 0.5),
        roughness: 0.0,
    };
    world.add(Box::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    //Camera
    let look_from = Vec3::new(13.0, 2.0, 3.0);
    let look_at = Vec3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let cam = Camera::new(
        look_from,
        look_at,
        vup,
        20.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );

    // ProgressBar
    let pb = ProgressBar::new(image_height as u64);

    // Timing
    let start = Instant::now();

    // Render
    let pixels = (0..image_height)
        .into_par_iter()
        .rev()
        .map(|j| {
            pb.inc(1);
            (0..image_width)
                // .into_par_iter()
                .map(|i| {
                    let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);
                    for _ in 0..num_samples {
                        let mut rng = rand::thread_rng();

                        let u = (i as f32 + rng.gen::<f32>()) / (image_width - 1) as f32;
                        let v = (j as f32 + rng.gen::<f32>()) / (image_height - 1) as f32;

                        let r = cam.get_ray(u, v);

                        pixel_color += ray_color(&r, &world, 0, max_depth);
                    }

                    let scale = 1.0 / num_samples as f32;
                    let r = (scale * pixel_color.x).sqrt();
                    let g = (scale * pixel_color.y).sqrt();
                    let b = (scale * pixel_color.z).sqrt();

                    format!(
                        "{} {} {}\n",
                        (r * 255.0) as i32,
                        (g * 255.0) as i32,
                        (b * 255.0) as i32
                    )
                })
                .collect::<Vec<String>>()
                .join("")
        })
        .collect::<Vec<String>>()
        .join("");

    // Timing
    pb.finish();
    let duration = start.elapsed();

    println!("Finished tracing in: {:?}\nWriting to file...", duration);

    // Write final render
    let mut pic = format!("P3\n{} {}\n{}\n", image_width, image_height, 255);
    pic = format!("{}{}", &pic, pixels);

    let file_name = format!(
        "output/{}.ppm",
        chrono::offset::Local::now().format("%Y-%m-%d %H-%M-%S")
    );
    std::fs::write(file_name, pic)?;

    Ok(())
}
