use ultraviolet::*;
use crate::ray::Ray;
use crate::hittable::*;

use rand::Rng;

#[derive(Clone, Copy, Debug)]
pub enum Material {
    Lambertian { albedo: Vec3 },
    Metal { albedo: Vec3, roughness: f32 },
    Dielectric { index_refraction: f32 }
}

impl Material {
    pub fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> (Ray, Vec3, bool) {
        match self {
            Material::Lambertian { albedo } => {
                let mut scatter_direction = rec.normal + random_unit_vector();
                if scatter_direction.near_zero() {
                    scatter_direction = rec.normal;
                }
                (Ray::new(rec.p, scatter_direction), *albedo, true)
            },
            Material::Metal { albedo, roughness } => {
                 let reflected = reflect(unit_vector(r_in.direction), rec.normal);
                 let scattered = Ray::new(rec.p, reflected + (*roughness) * random_in_unit_sphere());
                 let b = dot(scattered.direction, rec.normal) > 0.0;
                 return (scattered, *albedo, b)
            }
            Material::Dielectric { index_refraction } => {
                let (outward_normal, ni_over_nt, cosine) = if r_in.direction.dot(rec.normal) > 0.0 {
                    (
                        -rec.normal,
                        *index_refraction,
                        *index_refraction * r_in.direction.dot(rec.normal) / r_in.direction.length()
                    )
                } else {
                    (
                        rec.normal,
                        1.0 / *index_refraction,
                        -(r_in.direction.dot(rec.normal)) / r_in.direction.length()
                    )
                };
                
                let reflected = reflect(r_in.direction, rec.normal);

                let scattered = match refract(r_in.direction, outward_normal, ni_over_nt) {
                    Some(refracted) => {
                        let reflect_prob = schlick(cosine, *index_refraction);
                        let mut rng = rand::thread_rng();
                        if rng.gen::<f32>() < reflect_prob {
                            Ray::new(rec.p, reflected)
                        } else {
                            Ray::new(rec.p, refracted)
                        }
                    }
                    None => Ray::new(rec.p, reflected)
                };

                let attenuation = Vec3::new(1.0, 1.0, 1.0);

                return (scattered, attenuation, true);
                
            }
        }
    }
}


pub fn reflect(v1: Vec3, v2: Vec3) -> Vec3 {
    return v1 - 2.0 * v1.dot(v2) * v2
}

pub fn refract(v: Vec3, n: Vec3, ni_over_nt: f32) -> Option<Vec3> {
    let uv = unit_vector(v);
    let dt = dot(uv, n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if discriminant > 0.0 {
        return Some((uv - n * dt) * ni_over_nt - n * discriminant.sqrt());
    } else {
        return None;
    }
}

pub fn schlick(cosine: f32, ir: f32) -> f32 {
    let mut r0 = (1.0 - ir) / (1.0 + ir);
    r0 = r0 * r0;
    return r0 + (1.0 - r0) * ((1.0 - cosine).powf(5.0));
}