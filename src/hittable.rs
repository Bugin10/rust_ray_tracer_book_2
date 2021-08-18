use ultraviolet::*;
use crate::ray::Ray;
use crate::material::*;
use crate::aabb::*;

#[derive(Clone, Copy, Debug)]
pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f32
}

pub trait Hittable: Sync {
    fn hit(&self, r:&Ray, t_min: f32, t_max: f32) -> Option<(HitRecord, &Material)>;
    
    fn bounding_box(&self, time0: f32, time: f32, output_box: &mut AABB) -> bool;
}