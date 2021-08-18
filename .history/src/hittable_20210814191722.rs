use ultraviolet::*;
use crate::ray::Ray;
use crate::material::*;

#[derive(Clone, Copy, Debug)]
pub struct HitRecord {
    pub p: Vec3x8,
    pub normal: Vec3x8,
    pub t: f32
}

pub trait Hittable: Sync {
    fn hit(&self, r:&Ray, t_min: f32, t_max: f32) -> Option<(HitRecord, &Material)>;
}