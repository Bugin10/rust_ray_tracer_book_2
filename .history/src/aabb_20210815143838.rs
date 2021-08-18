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
        for a in 0..3 {
            let t0 = 
                ((self.minimum[a] - r.origin[a]) / r.direction[a])
                .min(
                (self.maximum[a] - r.origin[a]) / r.direction[a]);
            let t1 =
                ((self.minimum[a] - r.origin[a]) / r.direction[a])
                .max(
                    ((self.minimum[a] - r.origin[a]) / r.direction[a])
                );
            
        }
        true
    }
}

pub trait Hittable: Sync {
    fn hit(&self, r:&Ray, t_min: f32, t_max: f32) -> Option<(HitRecord, &Material)>;
}