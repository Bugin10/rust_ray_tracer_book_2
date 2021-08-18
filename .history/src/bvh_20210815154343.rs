use crate::aabb::*;
use crate::hittable::*;
use crate::hittable_list::*;
use crate::material::*;
use crate::ray::*;

use ultraviolet::*;

pub struct BVHNode {
    pub left: Box<dyn Hittable>,
    pub right: Box<dyn Hittable>,
    pub aabb_box: AABB,
}

impl BVHNode {
    pub fn new(hittable_list: &HittableList, time0: f32, time1: f32) -> BVHNode {
        new_node(hittable_list.objects, 0, hittable_list.objects.len(), time0, time1)
    }

    pub fn new_node(src_object: Vec<Box<dyn Hittable>>, start: usize, end: usize, time0: f32, time1: f32) -> BVHNode {
        BVHNode {
            AABB::new(Vec3::new(0.0,0.0,0.0), Vec3::new(0.0,0.0,0.0))
        }
    }

    pub fn center_at_t(&self, time: f32) -> Vec3 {
        self.center0
            + ((time - self.time0) / (self.time1 - self.time0)) * (self.center1 - self.center0)
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<(HitRecord, &Material)> {
        let oc = r.origin - self.center_at_t(r.time);
        let a = r.direction.mag_sq();
        let half_b = oc.dot(r.direction);
        let c = oc.mag_sq() - self.radius * self.radius;

        let discriminant = half_b.powf(2.0) - a * c;
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

        Some((rec, &self.material))
    }

    fn bounding_box(&self, time0: f32, time1: f32, output_box: &mut AABB) -> bool {
        let box0 = &mut AABB::new(
            self.center_at_t(time0) - Vec3::new(self.radius, self.radius, self.radius),
            self.center_at_t(time0) + Vec3::new(self.radius, self.radius, self.radius),
        );
        let box1 = &mut AABB::new(
            self.center_at_t(time1) - Vec3::new(self.radius, self.radius, self.radius),
            self.center_at_t(time1) + Vec3::new(self.radius, self.radius, self.radius),
        );
        *output_box = surrounding_box(&box0, &box1);
        true
    }
}
