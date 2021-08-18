use ultraviolet::*;
use crate::ray::Ray;

#[derive(Clone, Copy, Debug)]
pub struct AABB {
   pub minimum: Vec3,
   pub maximum: Vec3,
}

impl AABB {
    pub fn new(minimum: Vec3, maximum: Vec3) -> HitRecord {
        AABB{
            minimum:minimum,
            maximum:maximum,
        }
    }

    pub fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> bool {
        for i in 0..3 {
            let t0 = min((minimum.x - r.origin.x))
        }
    }
}

pub trait Hittable: Sync {
    fn hit(&self, r:&Ray, t_min: f32, t_max: f32) -> Option<(HitRecord, &Material)>;
}