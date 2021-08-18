use crate::hittable::*;
use crate::material::*;
use crate::ray::*;
use crate::aabb::*;

use ultraviolet::*;

pub struct Moving_Sphere {
    center0: Vec3,
    center1: Vec3,
    time0: f32,
    time1: f32,
    radius: f32,
    material: Material,
}

impl Moving_Sphere {
    pub fn new(center0: Vec3, center1: Vec3, time0: f32, time1: f32, radius: f32, material: Material) -> Moving_Sphere {
        Moving_Sphere {
            center0: center0,
            center1: center1,
            time0: time0,
            time1: time1,
            radius: radius,
            material: material,
        }
    }

    pub fn center_at_t(&self, time: f32) -> Vec3 { 
        return self.center0 + ((time - self.time0) / (self.time1 - self.time0)) * (self.center1 - self.center0);
    }
}

impl Hittable for Moving_Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<(HitRecord, &Material)> {
        let oc = r.origin - self.center_at_t(r.time);
        let a = r.direction.mag_sq();
        let half_b = oc.dot(r.direction);
        let c = oc.mag_sq() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let t = root;
        let p = r.at_parameter(root);
        let normal = (p - self.center_at_t(r.time)) / self.radius;

        let rec = HitRecord {
            t: t,
            p: p,
            normal: normal,
        };

        return Some((rec, &self.material));
    }

    fn bounding_box(&self, time0: f32, time1: f32, output_box: &mut AABB) -> bool {
        let box0 = &mut AABB::new(
            self.center_at_t(time0) - Vec3::new(self.radius, self.radius, self.radius),
            self.center_at_t(time0) + Vec3::new(self.radius, self.radius, self.radius)
        );
        let box1 = &mut AABB::new(
            self.center_at_t(time1) - Vec3::new(self.radius, self.radius, self.radius),
            self.center_at_t(time1) + Vec3::new(self.radius, self.radius, self.radius)
        );
        &output_box = surrounding_box(&box0, &box1);
        return true;
    }
}
