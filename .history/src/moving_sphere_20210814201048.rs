use ultraviolet::*;
use crate::ray::*;
use crate::hittable::*;
use crate::material::*;

pub struct Moving_Sphere {
    center: Vec3,
    radius: f32,
    material: Material
}

impl Moving_Sphere {
    pub fn new(center: Vec3, radius: f32, material: Material) -> Moving_Sphere {
        Moving_Sphere {
            center0: center0,
            center1: center1,
            radius: radius,
            material: material
        }
    }
}

impl Hittable for Moving_Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<(HitRecord, &Material)> {
        let oc = r.origin - self.center;
        let a = r.direction.mag_sq();
        let half_b = oc.dot(r.direction);
        let c = oc.mag_sq() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd  = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let t = root;
        let p = r.at_parameter(root);
        let normal = (p - self.center) / self.radius;

        let rec = HitRecord {
            t: t,
            p: p,
            normal: normal
        };

        return Some((rec, &self.material));
    }
}