use ultraviolet::*;
use crate::ray::Ray;

#[derive(Clone, Copy, Debug)]
pub struct HitRecord {
   pub minimum: Vec3,
   pub maximum: Vec3,
}

impl Moving_Sphere {
    pub fn new(minimum: Vec3, maximum: Vec3) -> HitRecord {
        minimum:minimum,
        
    }

pub trait Hittable: Sync {
    fn hit(&self, r:&Ray, t_min: f32, t_max: f32) -> Option<(HitRecord, &Material)>;
}