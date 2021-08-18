use ultraviolet::*;
use crate::ray::Ray;

#[derive(Clone, Copy, Debug)]
pub struct AABB {
   pub minimum: Vec3,
   pub maximum: Vec3,
}

impl AABB {
    pub fn new(minimum: Vec3, maximum: Vec3) -> AABB {
        AABB{
            minimum:minimum,
            maximum:maximum,
        }
    }

    pub fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> bool {
        for i in 0..3 {
            let t0 = (
                ((self.minimum.x - r.origin.x) / r.direction.x).min(
                (self.maximum.x - r.origin.x) / r.direction.x));
            let t1 = max (
                (self.minimum.y - r.origin.y) / r.direction.y
            );
        }
        true
    }
}

pub trait Hittable: Sync {
    fn hit(&self, r:&Ray, t_min: f32, t_max: f32) -> Option<(HitRecord, &Material)>;
}