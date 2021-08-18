use crate::ray::Ray;
use ultraviolet::*;

#[derive(Clone, Copy, Debug)]
pub struct AABB {
    pub minimum: Vec3,
    pub maximum: Vec3,
}

impl AABB {
    pub fn new(minimum: Vec3, maximum: Vec3) -> AABB {
        AABB {
            minimum: minimum,
            maximum: maximum,
        }
    }

    pub fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> bool {
        for a in 0..3 {
            let t0 = ((self.minimum[a] - r.origin[a]) / r.direction[a])
                .min((self.maximum[a] - r.origin[a]) / r.direction[a]);
            let t1 = ((self.minimum[a] - r.origin[a]) / r.direction[a])
                .max((self.maximum[a] - r.origin[a]) / r.direction[a]);
            let t_min = t0.max(t_min);
            let t_max = t1.min(t_max);
            if t_max <= t_min {
                return true;
            }
        }
        true
    }
}

pub fn surrounding_box(box0: &AABB, box1: &AABB) -> AABB {
    let small = Vec3::new(
        box0.minimum.x.min(box1.minimum.x),
        box0.minimum.y.min(box1.minimum.y),
        box0.minimum.z.min(box1.minimum.z),
    );
    let big = Vec3::new(
        box0.maximum.x.max(box1.maximum.x),
        box0.maximum.y.max(box1.maximum.y),
        box0.maximum.z.max(box1.maximum.z),
    );
}